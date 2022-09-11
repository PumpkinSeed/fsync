# fsync

File sync - Sync the files between multiple computers (like dotfiles, IDE files, etc.)

### Concept

- Sync files by using Git repository
- commands:
  - commit - commit the actual files into the git repository
  - sync - pull the changes and if any than sync
  - init - create the /etc folder structure (sudo)
  - remove - remove the /etc folder structure (sudo)
- Check the file changes by hashes, if it different (than change happened)
- Create a repository which does not changed by manually
- Config in the /etc folder

### Commit

1. Read the config file from /etc
2. Pull the repository into /tmp/{timestamp}
3. Check the integrity of files based on hash/checksum (md5)
4. If any file differes then copy that into the repository
5. Commit and push the changes with the timestamp

### Sync

1. Read the config file from /etc
2. Check the last commit, if it's newer than the one in the /etc than sync
3. Pull the repository into /tmp
4. Check the integrity of files based on hash/checksum (md5)
5. If any file differes then copy that into the location by overwriting the original
