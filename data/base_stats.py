from lxml import html
import cssselect

IN_FILE = 'in/base_stats.html'
OUT_FILE = '../src/pokemon_data/generated.rs'


def read_stats():
    with open(IN_FILE) as f:
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


def write_stats(stats):
    with open(OUT_FILE, 'w') as f:
        f.write('// Thie file was auto-generated using data/base_stats.py\n')
        f.write('use pokemon_data::PokemonData;\n')
        f.write('use types::PokeType;\n\n')

        f.write('pub const POKE_STATS: &\'static [PokemonData] = &[\n')

        for stat in stats:
            f.write('\tPokemonData {\n')

            f.write('\t\tid: {},\n'.format(stat['id']))

            f.write('\t\tattack: {},\n'.format(stat['attack']))
            f.write('\t\tdefense: {},\n'.format(stat['defense']))
            f.write('\t\tstamina: {},\n'.format(stat['stamina']))

            f.write('\t\tprimary_type: PokeType::{},\n'.format(stat['primary_type']))

            s_type = 'Some(PokeType::{})'.format(stat['secondary_type']) if stat['secondary_type'] else 'None'
            f.write('\t\tsecondary_type: {},\n'.format(s_type))

            f.write('\t},\n')

        f.write('];\n')

if __name__ == '__main__':
    poke_stats = read_stats()
    write_stats(poke_stats)
