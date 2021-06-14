# csv-generate-ids

A simple tool to generate unique sequential identifiers.

## Installation

You can use cargo to install this tool:

```bash
cargo install -f csv-generate-ids
```

## Usage

Here is an example CSV file.

```csv
title,album,artist,genre,country,released,duration,released-timestamp,duration-float
Östermalm,Stockholm,The Persuader,Electronic,Sweden,1999-03-00,4:45,920246400,4.45
Vasastaden,Stockholm,The Persuader,Electronic,Sweden,1999-03-00,6:11,920246400,6.11
Kungsholmen,Stockholm,The Persuader,Electronic,Sweden,1999-03-00,2:49,920246400,2.49
Södermalm,Stockholm,The Persuader,Electronic,Sweden,1999-03-00,5:38,920246400,5.38
Norrmalm,Stockholm,The Persuader,Electronic,Sweden,1999-03-00,4:52,920246400,4.52
Gamla Stan,Stockholm,The Persuader,Electronic,Sweden,1999-03-00,5:16,920246400,5.16
A Sea Apart,Knockin' Boots (Vol 2 Of 2),Mr. James Barth & A.D.,Electronic,Sweden,1998-06-00,5:08,896659200,5.08
Dutchmaster,Knockin' Boots (Vol 2 Of 2),Mr. James Barth & A.D.,Electronic,Sweden,1998-06-00,4:21,896659200,4.21
Inner City Lullaby,Knockin' Boots (Vol 2 Of 2),Mr. James Barth & A.D.,Electronic,Sweden,1998-06-00,4:22,896659200,4.22
Yeah Kid!,Knockin' Boots (Vol 2 Of 2),Mr. James Barth & A.D.,Electronic,Sweden,1998-06-00,4:46,896659200,4.46
```

If you want to output an array when CSV values are comma separated, specify those headers as arguments.

```bash
cat data.csv | csv-generate-ids --id-step-by 2 --id-start-at 6 --id-field-
name ObjectID
```

The output of the previous command would be something like so.

```csv
ObjectID,title,album,artist,genre,country,released,duration,released-timestamp,duration-float
6,Östermalm,Stockholm,The Persuader,Electronic,Sweden,1999-03-00,4:45,920246400,4.45
8,Vasastaden,Stockholm,The Persuader,Electronic,Sweden,1999-03-00,6:11,920246400,6.11
10,Kungsholmen,Stockholm,The Persuader,Electronic,Sweden,1999-03-00,2:49,920246400,2.49
12,Södermalm,Stockholm,The Persuader,Electronic,Sweden,1999-03-00,5:38,920246400,5.38
14,Norrmalm,Stockholm,The Persuader,Electronic,Sweden,1999-03-00,4:52,920246400,4.52
16,Gamla Stan,Stockholm,The Persuader,Electronic,Sweden,1999-03-00,5:16,920246400,5.16
18,A Sea Apart,Knockin' Boots (Vol 2 Of 2),Mr. James Barth & A.D.,Electronic,Sweden,1998-06-00,5:08,896659200,5.08
20,Dutchmaster,Knockin' Boots (Vol 2 Of 2),Mr. James Barth & A.D.,Electronic,Sweden,1998-06-00,4:21,896659200,4.21
22,Inner City Lullaby,Knockin' Boots (Vol 2 Of 2),Mr. James Barth & A.D.,Electronic,Sweden,1998-06-00,4:22,896659200,4.22
24,Yeah Kid!,Knockin' Boots (Vol 2 Of 2),Mr. James Barth & A.D.,Electronic,Sweden,1998-06-00,4:46,896659200,4.46
```
