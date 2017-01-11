# local dependency
import train_helper as helper
import json
import sqlite3

import pandas as pd
import numpy as np
import math


conn = sqlite3.connect('db/harvest.db')
c = conn.cursor()
c2 = conn.cursor()

# Do this instead
t = ('LOW52D',)
c.execute('SELECT stock FROM strategy_stk where  strategy=?', t)
# print c.fetchone()
scores = []
entries = []
for row in c:
	json_out = helper.train(row[0])
	scores.append(json_out['score'])
	entries.append(
			(t[0],
			row[0],
			0, # normalized_score
			round(json_out['score'],2),
			json_out['margin'],
			json_out['returns_median'],
			json_out['returns_mean'],
			json_out['returns_std'],
			json.dumps(json_out)))


scores_df = pd.DataFrame(scores);
# print scores_df
scores_df = scores_df.apply(np.sqrt)
# print scores_df
sum_var = scores_df.apply(np.sum)
# print sum_var
scores_df = scores_df.divide(sum_var)
# print scores_df

final_entries = []
idx = 0
for item in entries:
	conv_ls = list(item)
	conv_ls[2]= round(scores_df.get_value(idx,0),2)
	final_entries.append(tuple(conv_ls))
	idx=idx+1

# print final_entries

c.execute('delete from strategy_stk where strategy=?', t)
c.executemany('INSERT INTO strategy_stk VALUES (?,?,?,?,?,?,?,?,?)', final_entries)
conn.commit()

print 0