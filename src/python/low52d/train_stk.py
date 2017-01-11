#!/usr/bin/python
# -*- coding: utf-8 -*-
import json
import sys
import logging
import uuid

# local dependency

import train_helper as helper

logging.basicConfig(level=logging.DEBUG, filename='log/python.log',
                    filemode='w', format='%(message)s')

opt = {}

# simple JSON echo script


lines = sys.argv[1];
#print lines

print helper.train(lines)
# print json.dumps(msg)
