# Ensure aorist.so can be imported from repo root:
import sys, os.path
sys.path.append(os.path.join(os.path.dirname(__file__), '..', '..'))

import azure_covid, nyt_covid, who_covid, statsnz_covid, ECDC_covid, racial_covid_data_tracker 

# <filename>.<datasetname>
datasets = [
    azure_covid.azure_covid_dataset,
    ECDC_covid.ECDC_covid_dataset,
    nyt_covid.covid_dataset,
    statsnz_covid.statsnz_covid_dataset,
    who_covid.who_covid_dataset,
    racial_covid_data_tracker.trcdt_dataset
]
