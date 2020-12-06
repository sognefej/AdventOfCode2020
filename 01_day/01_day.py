with open('./input.txt') as input: 
    report = map(int, input.read().split())

def solutions_one(report, target):
    for x in report:
        y = target - x
        if y in report:
            return x * y 


def solution_two(report, target):
    for x in report: 
        for y in report:
            if x + y > target:
                continue
            z = target - (x + y)
            if z in report:
                return x * y * z

print(solutions_one(report, 2020))
print(solution_two(report, 2020))

# Solution one:
# x + y = 2020 
# so if we have one we have the other 
# y = 2020 - x 
# so if y is in the list we can return it. 

# Solution two: 
# same logic 
# x + y + z = 2020 
# so 
# z = 2020 - (x + y)
# if z happens to be in the report we have the solution 
# we can also ignore a combination if x + y > 2020 
# and is a lot more efficient than 
# for x in range:
#   for y in range:
#       for z in range:
#           if (x + y + z == 2020):
#               return x * y * z