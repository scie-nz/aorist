from aorist import aorist
from aorist import DownloadDataFromRemotePushshiftAPILocationToNewlineDelimitedJSON
from json import dumps

programs = {}

@aorist(
    programs,
    DownloadDataFromRemotePushshiftAPILocationToNewlineDelimitedJSON,
    entrypoint="download_subreddit",
    args={
        "subreddit": lambda pushshift_api_location: pushshift_api_location.subreddit,
        "tmp_dir": lambda replication_storage_setup: replication_storage_setup.tmp_dir,
        "output_file": lambda replication_storage_setup, pushshift_api_location, context: (
            context.capture(
                "file_to_replicate", replication_storage_setup.tmp_dir + "/" + pushshift_api_location.subreddit + ".json"
            ),
            context
        ),
        "_is_json": lambda context: (context.capture("is_json", dumps(True)), context),
        "_delimiter": lambda context: (context.capture("delimiter", dumps(None)), context),
        "_header_num_lines": lambda context: (context.capture("header_num_lines", dumps(0)), context),
    },
)
def recipe(subreddit, tmp_dir, output_file, _is_json, _delimiter):
    from pmaw import PushshiftAPI
    import json
    import os

    def download_subreddit(subreddit, tmp_dir, output_file, _is_json, _delimiter, _header_num_lines):

        if not os.path.exists(tmp_dir):
            os.makedirs(tmp_dir)

        if not os.path.exists(output_file) or os.stat(output_file).st_size == 0:
            api = PushshiftAPI(
                num_workers=16,
                limit_type='backoff',
                jitter='decorr',
            )
            posts = api.search_submissions(
                subreddit=subreddit,
                limit=10e9,
                after=0
            )
            with open(output_file, 'w') as f:
              for post in posts:
                  line = json.dumps(post) + chr(10)
                  f.write(line)


