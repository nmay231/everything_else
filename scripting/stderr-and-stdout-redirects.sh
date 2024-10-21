# Basic redirects
comm > overwrite_file_with_stdout
comm >> append_to_file_with_stdout
comm | tee file | comm2 # Write to file and pipe to comm2

# `> file` is the same as `1> file`
comm > write_stdout 2> write_stderr

# Merging stderr and stdout
comm &> overwrite_file_with_both_stdout_and_stderr
comm >> merge_stdout_and_stderr_into_the_same_file_descriptor 2>&1 # Also useful if you decide you want to grep output

# Doesn't work! Basically because the file descriptors are changed from left to right, so the 2>&1 does nothing since stderr already was the same as stdout
comm 2>&1 >> file

# Helpful resource: https://web.archive.org/web/20230315225157/https://wiki.bash-hackers.org/howto/redirection_tutorial
