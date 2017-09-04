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
        defense = node.cssselect('.progress')[2].get('title')

        stats = {
            'id': node.get('data-species-num'),
            'attack': node.get('data-base-attack'),
            'defense': defense,
            'stamina': node.get('data-base-stamina'),
            'max_cp': node.get('data-max-cp')
        }

        vec.append(stats)

    vec.sort(key=lambda d: int(d['id']))

    return vec


def write_stats(stats):
    with open(OUT_FILE, 'w') as f:
        f.write('// Thie file was auto-generated using data/base_stats.py\n')
        f.write('use pokemon_data::PokemonData;\n\n')

        f.write('pub const POKE_STATS: &\'static [PokemonData] = &[\n')

        for stat in stats:
            f.write('\tPokemonData {\n')

            f.write('\t\tid: {},\n'.format(stat['id']))
            f.write('\t\tattack: {},\n'.format(stat['attack']))
            f.write('\t\tdefense: {},\n'.format(stat['defense']))
            f.write('\t\tstamina: {}\n'.format(stat['stamina']))

            f.write('\t},\n')

        f.write('];\n')

if __name__ == '__main__':
    poke_stats = read_stats()
    write_stats(poke_stats)
