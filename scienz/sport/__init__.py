# Ensure aorist.so can be imported from repo root:
import sys, os.path
sys.path.append(os.path.join(os.path.dirname(__file__), '..', '..'))

import mls_usa, nfl_weather, nfl_2020_pbp, nba_2020_pbp, int_1800_football, english_premier_league, bundesliga

# filename.datasetname
datasets = [
    mls_usa.mls_usa_dataset,
    nfl_weather.nfl_weather_dataset,
    nfl_2020_pbp.nfl_2020_pbp_dataset,
    nba_2020_pbp.nba_2020_pbp_dataset,
    int_1800_football.int_1800_football_dataset,
    english_premier_league.english_premier_league_dataset,
    bundesliga.bundesliga_dataset
]
