// Handle creating new file version
//  Check last version of file
//  If it was checked out, do not allow
//  Increment version for this one
//  If everything checks out, make and return an UploadId for the file
//  


// UploadId system
// After all metadata is stored, create an upload id for the file
// Use this as a url param to upload the file with streaming to something like S3
// UploadId is a UUID