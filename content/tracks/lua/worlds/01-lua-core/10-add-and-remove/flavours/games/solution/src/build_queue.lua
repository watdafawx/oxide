local M = {}

-- put job at the end of queue
function M.add_job(queue, job)
  table.insert(queue, job)
end

-- remove slot i, close the gap, and return what was there
function M.finish_job(queue, i)
  return table.remove(queue, i)
end

return M
