from aorist import aorist, DownloadDataFromRemotePushshiftAPILocationToNewlineDelimitedJSON

programs = {}

@aorist(
    programs,
    DownloadDataFromRemotePushshiftAPILocationToNewlineDelimitedJSON,
    entrypoint="download_subreddit",
    args={
        "subreddit": lambda lng: lng.pushshift_api_location.subreddit,
        "tmp_dir": lambda lng: lng.replication_storage_setup.tmp_dir,
    },
)
def recipe(subreddit, tmp_dir):
    from pmaw import PushshiftAPI
    import json
    import os
    
    def download_subreddit(subreddit, tmp_dir):
    
        if not os.path.exists(tmp_dir):
            os.makedirs(tmp_dir)
    
        output_file = '%s/data.json' % tmp_dir
    
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
    
    