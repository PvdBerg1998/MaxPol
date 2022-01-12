# MaxPol

MaxPol is a lowpass derivative kernel optimized to have "ideal" spectral properties. See literature: https://arxiv.org/abs/1709.08321, https://ieeexplore.ieee.org/document/7944698 for more details. Unfortunately there is no closed form for the kernel with general parameters. I present analytical (exact) solutions for the centralized MaxPol kernel, with `s=0` (no shift). Implements the same algorithm as the original [solver](https://github.com/mahdihosseini/MaxPol), but in Mathematica instead of Matlab. Shifting can be implemented trivially if required.

## This repository

This repository contains the solutions for `n=0..4`, `l=1..100` (i.e. zero'th to fourth derivatives with a filter length up to 201) and all possible `P,Q` values (degree of lowpass filtering vs. tangency to exact derivative). The `n=0` setting corresponds to filtering only.

[maxpol_calc](/maxpol_calc.tar.gz) contains numerical values, truncated at 18 significant digits (more than `f64` precision). Each value is printed on a newline, starting from the leftmost filter coefficient. Each solution is named `maxpol_n<n>_l<l>_P<P>`.

[maxpol_calc_exact](/maxpol_calc_exact.tar.gz) contains all the exact fractions formatted in the same way.

It is recommended to make use of an intermediate storage format, such as the Sled database as implemented in [maxpol_rust](/maxpol_rust).

The approximate maxima of the spectra are stored in [maxpol_calc_cutoff](/maxpol_calc_cutoff.tar.gz), to be used as an indication of the cutoff frequency.

## License

Everything is released under the GPL-3.0 license. If you use my work in academic context, a citation would be appreciated:
```latex
@misc{maxpol_pvdb,
  author = {Pim van den Berg},
  title = {MaxPol analytical solutions up to l=100},
  howpublished = {https://github.com/PvdBerg1998/MaxPol/}
  year = {2022}
}
```
