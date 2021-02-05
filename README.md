# splitfa

Splits a FASTA file into segments of the given length with a defined step size between segment starts.

## usage

This would write all 2kbp sequences starting every 1kbp:

```
splitfa seqs.fa -l 2000 -s 1000 >splits.fa
```
