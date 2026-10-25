param([string]$Exe, [int]$Settle = 8, [int]$Window = 20)
function Tree($rootId) {
  $all = Get-CimInstance Win32_Process
  $ids = @($rootId); $grew = $true
  while ($grew) { $grew = $false
    foreach ($p in $all) { if ($ids -contains $p.ParentProcessId -and -not ($ids -contains $p.ProcessId)) { $ids += $p.ProcessId; $grew = $true } } }
  $ids | ForEach-Object { Get-Process -Id $_ -ErrorAction SilentlyContinue }
}
$sw = [Diagnostics.Stopwatch]::StartNew()
$p = Start-Process -FilePath $Exe -PassThru
while ($p.MainWindowHandle -eq 0 -and $sw.ElapsedMilliseconds -lt 15000) { Start-Sleep -Milliseconds 20; $p.Refresh() }
$winMs = $sw.ElapsedMilliseconds
Start-Sleep -Seconds $Settle
$procs = Tree $p.Id
Start-Sleep -Seconds $Window
$procs = Tree $p.Id
$cpu1 = 0; foreach ($q in $procs) { $q.Refresh(); $cpu1 += $q.TotalProcessorTime.TotalMilliseconds }
$ws = ($procs | Measure-Object WorkingSet64 -Sum).Sum / 1MB
$priv = ($procs | Measure-Object PrivateMemorySize64 -Sum).Sum / 1MB
$cores = [Environment]::ProcessorCount
"{0}: window after {1} ms, processes {2}, working set {3:N0} MB, private {4:N0} MB, idle CPU {5:N2}% of one core over {6}s" -f (Split-Path $Exe -Leaf), $winMs, $procs.Count, $ws, $priv, (100*($cpu1-$cpu0)/($Window*1000)), $Window
foreach ($q in ($procs | Sort-Object Id -Descending)) { Stop-Process -Id $q.Id -Force -ErrorAction SilentlyContinue }
