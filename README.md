# TW Hya Data Analysis & Figures

This repository contains the data and analysis scripts for observations of the protoplanetary disk around TW Hya.

## Data Access

Raw data for TW Hya is available through the [MAST portal for JWST](https://mast.stsci.edu/search/ui/#/jwst).

## Software Requirements

To run the analysis notebooks, you will need the following software packages:

*   **spaceKLIP**: This package is essential for processing the JWST data. You can find its documentation [here](https://spaceklip.readthedocs.io/en/latest/).
*   **Winnie**: This package is required for the `*_deconvo.ipynb` notebooks. [here](https://github.com/kdlawson/Winnie)

## Processing Workflow

The analysis follows a specific order of execution:

1.  **`*_cal.ipynb`**: These notebooks perform the initial calibration of the JWST data.
2.  **`*_deconvo.ipynb`**: These notebooks apply HPFRDI and deconvolution techniques to the calibrated data.
3.  **`*_planetmass.ipynb`**: These notebooks are used for estimating planet masses within the TW Hya system.
4.  **`plot.ipynb` & `plot_hprdi.ipynb`**: These notebooks are for generating plots and visualizations of the results.

## Notes

*   The TW Hya system is a well-studied T Tauri star with a significant protoplanetary disk, making it an ideal target for studying planet formation.
*   Recent studies have focused on detecting and characterizing planets within the TW Hya disk, including their masses and formation processes.
