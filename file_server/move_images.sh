#!/bin/bash

# Define the container name
CONTAINER_NAME="inmobify-services-file_server-1"

# Loop through img1.jpg to img10.jpg
for i in {1..10}; do
    # Define the source file path (relative to the script location)
    SOURCE_FILE="example_images/img$i.jpg"
    
    # Define the destination path inside the container
    DEST_FILE="/images/img$i.jpg"
    
    # Check if the source file exists
    if [ -f "$SOURCE_FILE" ]; then
        echo "Copying $SOURCE_FILE to $CONTAINER_NAME:$DEST_FILE"
        docker cp "$SOURCE_FILE" "$CONTAINER_NAME:$DEST_FILE"
        
        # Check if the copy was successful
        if [ $? -eq 0 ]; then
            echo "Successfully copied $SOURCE_FILE"
        else
            echo "Failed to copy $SOURCE_FILE"
            exit 1
        fi
    else
        echo "File $SOURCE_FILE does not exist, skipping..."
    fi
done

echo "Done!"