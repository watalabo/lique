# Generate Python test cases from file names which are not tracked by git yet
set -e

# Get the base file name without extension
modules=$(git ls-files --others --exclude-standard | sed -n -E 's|dataset/python/(.*)\.py|\1|p')

echo "${modules[@]}" | awk '{print "import python." $0 }'
echo
echo "${modules[@]}" | awk '{print "DatasetCase(\"ql-operation-after-measurement\", \"TP\", \"" $0 "\", python." $0 ".create_circuit)," }'

# for base_name in "${base_names[@]}"; do
#     awk '{print "DatasetCase(\"ql-operation-after-measurement\", \"TP\", \"" $0 "\", python." $0 ".create_circuit)," }'
# done
