The payroll has to pay everyone at the end of the month. One way is to ask each person "are you salaried? hourly?" and do the right sum. That code breaks the day the company hires its first intern, and someone has to remember to add a third question.

**An array of objects.** An array can hold objects as well as numbers. `Employee[] staff = { new Salaried("Ann", 60000), new Hourly("Bo", 20, 160) };` holds both kinds, because each of them is an `Employee`.

**Just ask.** Loop over the array and call `monthlyPay()` on each one. Java runs each object's own version: the salaried sum for Ann, the hourly sum for Bo. That is **polymorphism**: one call, and the object decides what it means:

```java
int meals = 0;
for (Pet pet : pets) {
    meals += pet.mealsPerDay();    // a Cat's 3, a Fish's 1
}
```

**Why it lasts.** The loop never names a kind. An `Intern` class written next year, extending `Employee`, is paid by the same loop with no change to `Payroll`. The checker's second objective slips in exactly such a class.

**The habit.** When you are about to ask an object what kind it is, ask instead whether the parent should have a method for it. Then each child answers for itself.

## Your turn

- In `Payroll.total`, loop over `staff` and add up everyone's `monthlyPay()`.
- Save. Both objectives turn green.
