from lxml import html
import cssselect

IN_FILE = 'in/type_effectiveness.html'
OUT_FILE = '../src/types/generated.rs'


def read_effectiveness():
    with open(IN_FILE) as f:
        tree = html.fromstring(f.read())

    tbody = tree.cssselect('tbody')[0]

    chart = []

    # Collect the rows, skipping the first empty row and the type icon row
    rows = tbody.cssselect('tr')[2:]

    for tr in rows:
        row = []

        for td in tr.cssselect('td')[1:]:
            if 'matrixfact1000' in td.classes:
                val = 'Normal'
            elif 'matrixfact510' in td.classes:
                val = 'DoubleNotVery'
            elif 'matrixfact714' in td.classes:
                val = 'NotVery'
            elif 'matrixfact1400' in td.classes:
                val = 'SuperEffective'

            assert val
            row.append('Effectiveness::{}'.format(val))

        chart.append(row)

    return chart


def write_effectiveness(chart):
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
    effectiveness_chart = read_effectiveness()
    write_effectiveness(effectiveness_chart)
