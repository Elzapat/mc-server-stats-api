import json
import glob
import os
import sys
import requests

# Script to get a stat for every player that has logged on the server
# Usage: python get_stat.py stat_type stat_value formatting
# Example : python get_stat.py minecraft:killed minecraft:creeper "{} has killed {} creepers"

path = "stats/"
stat_type = sys.argv[1]
stat_value = sys.argv[2]
output_format = sys.argv[3]
for filename in glob.glob(os.path.join(path, "*.json")):
    with open(filename) as json_file:
        stats = json.load(json_file)

        try:
            stat = stats["stats"][stat_type][stat_value]
        except KeyError:
            stat = 0

        uuid = filename[filename.find('/') + 1 : filename.find('.')]
        res = requests.get("https://api.mojang.com/user/profiles/" + uuid + "/names")
        names = json.loads(res.text)

        name = names[len(names) - 1]["name"]

        print(output_format.format(name, stat))
