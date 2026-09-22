# The ontoparse CLI

## xrefs

Extract cross-references (xrefs) from ontologies.

- Arguments
  - `to`: An anchor prefix to map from (e.g, `MESH`).
  - `from`: An anchor prefix to map from (e.g., `UBERON`). Must contain term entries in the file. Defaults to the most frequent prefix in the file.
  - `separator`: Delimiter of the output table. Default is `\t`.

```bash
ontoparse xref <FILE> --from <PREFIX> --to <PREFIX>
```

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
