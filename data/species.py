import csv

from lxml import html
import cssselect

NAMES_FILE = 'in/pokemon_names.tsv'
BASE_STATS_FILE = 'in/base_stats.html'
OUT_FILE = '../src/species/generated_species.rs'

NAME_EXCEPTIONS = {
    'Nidoran♀': 'NidoranFemale',
    'Nidoran♂': 'NidoranMale',
    'Farfetch\'d': 'Farfetchd',
    'Mr. Mime': 'MrMime',
    'Ho-Oh': 'HoOh'
}


def read_names():
    with open(NAMES_FILE, encoding='utf-8') as f:
        reader = csv.DictReader(f, delimiter='\t')
        return list(row for row in reader)


def read_stats():
    with open(BASE_STATS_FILE) as f:
        page = f.read()

    tree = html.fromstring(page)

    vec = []
    for node in tree.cssselect('.speciesWrap'):
        type_nodes = node.cssselect('.monTypes > div')
        assert len(type_nodes) in [1, 2]

        primary_type = type_nodes[0].text.capitalize()

        if len(type_nodes) == 2:
            secondary_type = type_nodes[1].text.capitalize()
        else:
            secondary_type = None

        defense = node.cssselect('.progress')[2].get('title')

        stats = {
            'id': node.get('data-species-num'),
            'attack': node.get('data-base-attack'),
            'defense': defense,
            'stamina': node.get('data-base-stamina'),
            'max_cp': node.get('data-max-cp'),
            'primary_type': primary_type,
            'secondary_type': secondary_type
        }

        vec.append(stats)

    vec.sort(key=lambda d: int(d['id']))

    return vec


def write_species(stats, names):
    with open(OUT_FILE, 'w', encoding='utf-8') as f:
        f.write('// This file was auto-generated using data/species.py\n')
        f.write('#![allow(non_upper_case_globals)]\n')
        f.write('use species::PokeSpecies;\n')
        f.write('use types::PokeType;\n\n')

        for stat, name in zip(stats, names):
            display_name = NAME_EXCEPTIONS.get(name['English'], name['English'])

            f.write('pub const {}: &\'static PokeSpecies = &PokeSpecies  {{\n'.format(display_name))

            f.write('\tid: {},\n'.format(stat['id']))

            f.write('\tattack: {},\n'.format(stat['attack']))
            f.write('\tdefense: {},\n'.format(stat['defense']))
            f.write('\tstamina: {},\n'.format(stat['stamina']))

            f.write('\tprimary_type: PokeType::{},\n'.format(stat['primary_type']))

            s_type = 'Some(PokeType::{})'.format(stat['secondary_type']) if stat['secondary_type'] else 'None'
            f.write('\tsecondary_type: {},\n'.format(s_type))

            f.write('\tenglish: "{}",\n'.format(name['English']))
            f.write('\tjapanese: "{}",\n'.format(name['Japanese']))
            f.write('\tjapanese_transliterated: "{}",\n'.format(name['JapaneseTransliterated']))
            f.write('\tkorean: "{}",\n'.format(name['Korean']))
            f.write('\tchinese: "{}",\n'.format(name['Chinese']))
            f.write('\tfrench: "{}",\n'.format(name['French']))
            f.write('\tgerman: "{}",\n'.format(name['German']))
            f.write('\tspanish: "{}",\n'.format(name['Spanish']))
            f.write('\titalian: "{}",\n'.format(name['Italian']))

            f.write('};\n\n')

        f.write('pub const ALL_SPECIES: &\'static [&\'static PokeSpecies] = &[\n')

        for name in names:
            display_name = NAME_EXCEPTIONS.get(name['English'], name['English'])

            f.write('\t{},\n'.format(display_name))

        f.write('];\n')

if __name__ == '__main__':
    poke_stats = read_stats()
    poke_names = read_names()

    write_species(poke_stats, poke_names)
