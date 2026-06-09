# whex

Hex dump library for Python, written in Rust.

Reads a binary file and returns its hex representation as a list of strings. Each line contains an offset, hex bytes, and printable ASCII characters.

---

## Installation

```bash
pip install whexx
```

---

## Usage

```python
import whex

lines = whex.create_dump("file.bin")
for line in lines:
    print(line)
```

Output:

```
00000000: 7F 45 4C 46 02 01 01 00 00 00 00 00 00 00 00 00  | .ELF............
00000010: 02 00 3E 00 01 00 00 00 00 10 40 00 00 00 00 00  | ..>.......@.....
```

---

## API

### `create_dump(filename, row_size=16, max_lines=None)`

| Parameter   | Type          | Default | Description                       |
|-------------|---------------|---------|-----------------------------------|
| `filename`  | `str`         | —       | Path to the file                  |
| `row_size`  | `int`         | `16`    | Number of bytes per line          |
| `max_lines` | `int \| None` | `None`  | Maximum number of lines to return |

Returns `list[str]`.

---

## Requirements

- Python 3.8+

---

## License

MIT

---

---

# whex

Библиотека hex-дампа для Python, написанная на Rust.

Читает бинарный файл и возвращает его hex-представление в виде списка строк. Каждая строка содержит смещение, байты в hex и печатаемые ASCII-символы.

---

## Установка

```bash
pip install whexx
```

---

## Использование

```python
import whex

lines = whex.create_dump("file.bin")
for line in lines:
    print(line)
```

Вывод:

```
00000000: 7F 45 4C 46 02 01 01 00 00 00 00 00 00 00 00 00  | .ELF............
00000010: 02 00 3E 00 01 00 00 00 00 10 40 00 00 00 00 00  | ..>.......@.....
```

---

## API

### `create_dump(filename, row_size=16, max_lines=None)`

| Параметр    | Тип           | По умолчанию | Описание                             |
|-------------|---------------|--------------|--------------------------------------|
| `filename`  | `str`         | —            | Путь к файлу                         |
| `row_size`  | `int`         | `16`         | Количество байт в одной строке       |
| `max_lines` | `int \| None` | `None`       | Максимальное количество строк вывода |

Возвращает `list[str]`.

---

## Требования

- Python 3.8+

---

## Лицензия

MIT
