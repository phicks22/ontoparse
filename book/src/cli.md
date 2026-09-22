# The ontoparse CLI

## xrefs

Extract cross-references (xrefs) from ontologies.

```bash
ontoparse xref <FILE> --to <PREFIX> [--from <PREFIX>] [--separator <SEP>]
```

- **Arguments**

  - `<FILE>`: An ontology OBO file in release format >1.2 or <1.4.

- **Required options**

  - `--to <PREFIX>`: An anchor prefix to map from (e.g, `MESH`).

- **Optional arguments**
  - `--from <PREFIX>`: An anchor prefix to map from (e.g., `UBERON`). Must contain term entries in the file. Defaults to the most frequent prefix in the file.
  - `--separator <SEP>`: Delimiter of the output table. Defaults to `\t`.

### Example

From the primary prefix (i.e., UBERON):

```bash
ontoparse xref uberon.obo --to MESH
```

Or you can specify:

```bash
ontoparse xref uberon.obo --from UBERON --to MESH
```

You can also map from other prefixes if they exist:

```bash
ontoparse xref uberon.obo --from CL --to MESH
```

Write the output to tsv:

```bash
ontoparse xref uberon.obo --to MESH > uberon_to_mesh.tsv
```

You may also change the separator:

```bash
ontoparse xref uberon.obo --to MESH --separator "," > uberon_to_mesh.csv
```
