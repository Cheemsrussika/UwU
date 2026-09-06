import sys
path = sys.argv[1]
data = sys.stdin.read()
with open(path, 'w', encoding='utf-8') as out:
    out.write(data)
