# TW Hya Data Analysis & Figures

This repository contains the data and analysis scripts for observations of the protoplanetary disk around TW Hya.

## Data Access

Raw data for TW Hya is available through the [MAST portal for JWST](https://mast.stsci.edu/search/ui/#/jwst).

## Software Requirements

To run the analysis notebooks, you will need the following software packages:

*   **spaceKLIP**: This package is essential for processing the JWST data. You can find its documentation [here](https://spaceklip.readthedocs.io/en/latest/).
*   **Winnie**: This package is required for the `*_deconvo.ipynb` notebooks. You can find its documentation [here](https://github.com/kdlawson/Winnie).
*   **Toy_Diskgeofitter**: This package is used for the `TDGF_twhya.ipynb` notebook. You can find its documentation [here](https://github.com/dreamjade/Toy_Diskgeofitter).

## Processing Workflow

The analysis follows a specific order of execution:

1.  **`*_cal.ipynb`**: These notebooks perform the calibration of the JWST raw data.
2.  **`*_deconvo.ipynb`**: These notebooks apply HPFRDI and deconvolution algorithm to the calibrated data.
3.  **`*_planetmass.ipynb`**: These notebooks are used for estimating planet mass sensitivities within the TW Hya system.
4.  **`*_model.ipynb`**: These notebooks are used for building disk models by fitting the HPFRDI image.
5.  **`*_model_error.ipynb`**: These notebooks are used for finding the best iteration number for the deconvolution algorithm.
6.  **`TDGF_twhya.ipynb`**: This notebook is used for finding disk geometry parameters by performing the Toy_Diskgeofitter analysis.
7.  **`twhya_cornerplot.ipynb`**: This notebook is used for generating corner plots of the disk geometry parameters.
8.  **`plot.ipynb`**: This notebook is for generating plots and visualizations of the results.
9.  **`Photometry_rust.ipynb`** in Photometry_rust folder: This notebooks are for performing photometric analysis on the processed data.
10. **`Photometry.ipynb`**: This notebooks are for performing photometric analysis on TW Hya system.

## Notes
*   **`twhya_sampler_*.pkl`** are the MCMC sampler results from the Toy_Diskgeofitter analysis (**`TDGF_twhya.ipynb`**). They are avilable in the release page of this repository. You can download them and put them in the appropriate directory (**`all_fit_*`** folder) to generate corner plots (**`twhya_cornerplot.ipynb`**) without running the Toy_Diskgeofitter analysis again.