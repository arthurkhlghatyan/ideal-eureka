'''
  Real world example:
  We have 20 students in a class and we have data about a specific exam they have taken.

'''

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt


students = [
  (29, 65),
  (9 , 7),
  (10, 8),
  (38, 76),
  (16, 23),
  (26, 56),
  (50, 100),
  (10, 3),
  (30, 74),
  (33, 48),
  (43, 73),
  (2 , 0),
  (39, 62),
  (15, 37),
  (44, 74),
  (29, 40),
  (41, 90),
  (15, 42),
  (24, 58),
  (50, 100),
]

student_data = pd.DataFrame(data=students, columns=['hours', 'results'])

x = student_data.hours
y = student_data.results

plt.scatter(x, y)

model = np.polyfit(x, y, 1)

print(model)
