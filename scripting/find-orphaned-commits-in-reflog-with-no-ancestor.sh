# One-liner
BRANCH=master; for HASH in $(git reflog | awk -F " " '{print $1}'); do {git merge-base --is-ancestor $HASH $BRANCH; if [ 1 -eq $? ]; then {echo -e "\n\n\n\n\n\n\n\n================="; git show $HASH}; fi}; done | less
# TODO: I think there's something here that only works in zsh, not bash for some reason. Don't want to investigate yet.

# Explained version with more options

BRANCH=prod
BRANCH2=dev

HASHES = $(
  git reflog
  # TODO: Figure out how to correctly merge these into one awk command. awk is sorta too powerful
  | tail -n +10 # Skip the first 10 commits
  | head -n -10 # Skip the last 10
  | awk -F " " '{print $1}'
  | awk '!seen_lines[$0]++' # Remove duplicate hashes
  # Source: https://stackoverflow.com/questions/11532157/remove-duplicate-lines-without-sorting
)

for HASH in $HASHES; do
  # The next line errors is none of the branches contain the hash
  git merge-base --is-ancestor $HASH $BRANCH || git merge-base --is-ancestor $HASH $BRANCH2;
  if [ 1 -eq $? ]; then
    echo -e "\n\n\n\n\n\n\n\n================="; # Some spacing. Adjust to personal taste
    git show $HASH;
  fi;
done
