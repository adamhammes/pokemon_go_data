from lxml import html
import cssselect

IN_FILE = 'in/type_effectiveness.html'
OUT_FILE = '../src/types/generated.rs'


def main():
    with open(IN_FILE) as f:
        tree = html.fromstring(f.read())

    tbody = tree.cssselect('tbody')[0]

    chart = []

    for tr in tbody.cssselect('tr')[1:]:
        row = []

        for td in tr.cssselect('td')[1:]:
            text = td.text.strip() if td.text else ''

            if not text:
                val = 'Normal'
            elif text.startswith('0.51'):
                val = 'DoubleNotVery'
            elif text.startswith('0.74'):
                val = 'NotVery'
            elif text.startswith('1.4'):
                val = 'SuperEffective'

            assert val
            row.append('Effectiveness::{}'.format(val))

        chart.append(row)

    with open(OUT_FILE, 'w') as f:
        f.write('// This file was auto-generated using data/type_effectiveness.py\n')
        f.write('use types::Effectiveness;\n\n')

        f.write("pub const EFFECTIVENESS_CHART: &'static [&'static [Effectiveness]] = &[\n")

        for row in chart:
            f.write('\t&[\n')

            for value in row:
                f.write('\t\t{},\n'.format(value))

            f.write('\t],\n')

        f.write('];\n')

if __name__ == '__main__':
    main()
