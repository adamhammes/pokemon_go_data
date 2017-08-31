import csv
import requests
from lxml import html
import cssselect

res = requests.get('https://thesilphroad.com/species-stats')
tree = html.fromstring(res.content)

vec = []
for node in tree.cssselect('.speciesWrap'):
    defense = node.cssselect('.progress')[2].get('title')

    d = {
        'id': node.get('data-species-num'),
        'attack': node.get('data-base-attack'),
        'defense': defense,
        'stamina': node.get('data-base-stamina'),
        'max_cp': node.get('data-max-cp')
    }

    vec.append(d)

with open('pokemon_stats.tsv', 'w', newline='') as csvfile:
    fieldnames = ['id', 'attack', 'defense', 'stamina', 'max_cp']
    writer = csv.DictWriter(csvfile, fieldnames=fieldnames)

    writer.writeheader()

    for row in vec:
        print(row)
        writer.writerow(row)


