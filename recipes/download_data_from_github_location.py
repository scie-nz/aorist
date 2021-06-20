from aorist import aorist, DownloadDataFromGithubLocation

programs = {}

@aorist(
    programs,
    DownloadDataFromGithubLocation,
    entrypoint="download_data_from_github_location",
    args={
        "organization": lambda lng: lng.match remote_storage.location {
  crate::location::RemoteLocation::GithubLocation(ref g) => g.organization,
  _ => panic!("Only GithubLocation supported"),
}
,
        "repository": lambda lng: lng.match remote_storage.location {
  crate::location::RemoteLocation::GithubLocation(ref g) => g.repository,
  _ => panic!("Only GithubLocation supported"),
}
,
        "branch": lambda lng: lng.match remote_storage.location {
  crate::location::RemoteLocation::GithubLocation(ref g) => g.branch,
  _ => panic!("Only GithubLocation supported"),
}
,
        "path": lambda lng: lng.match remote_storage.location {
  crate::location::RemoteLocation::GithubLocation(ref g) => g.path,
  _ => panic!("Only GithubLocation supported"),
}
,
        "tmp_dir": lambda lng: lng.replication_storage_setup.tmp_dir,
        "dataset_name": lambda lng: lng.data_set.name,
        "dest_file_name": lambda lng: lng.{
  format!(
      "{}.{}",
      static_data_table.name,
      match static_data_table.setup {
           StorageSetup::ReplicationStorageSetup(ref x) => x.get_download_extension(),
           _ => panic!("Storage setup must be ReplicationStorageSetup"),
      }
  )
}
,
        "table_name": lambda lng: lng.static_data_table.name,
    },
)
def recipe(organization, repository, branch, path, tmp_dir, dataset_name, dest_file_name, table_name):
    import os
    import urllib.request
    
    def download_data_from_github_location(
        organization,
        repository,
        branch,
        path,
        tmp_dir,
        dataset_name,
        dest_file_name,
        table_name,
    ):
       os.makedirs(tmp_dir + '/' + dataset_name + '/' + table_name, exist_ok=True)
       clone_url = (
          "https://raw.githubusercontent.com/{org}/{repo}/{branch}/{filename}"
       ).format(
          org=organization,
          repo=repository,
          branch=branch,
          filename=path,
       )
       print("Downloading data from: %s" % clone_url)
       urllib.request.urlretrieve(clone_url, os.path.join(tmp_dir, dataset_name, table_name, dest_file_name))
    
    