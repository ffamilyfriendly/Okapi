// This code is meant to handle multipart media (partials, http 206). I have no clue where to start but lesss do this thing

/*

    What i know: 
    request will contain a Range header. On firefox at least it looks like "bytes=<nr>-" where <nr> is the byte to start from. 
    It appears that firefox does not include a end to said range but other browsers might. 
    also contains an Accept header which shows what formats are accepted. How about we just ignore that lol. You ask for file, you get file

    Reponse shall contain headers:
    content-range which looks like "<nr> - <end> / <length>" where <nr> is start <end> is to what byte was sent and <length> is the length of the file.
    content-length which is how many bits of data is sent. <end> subtracted by <nr>
    accept-ranges "bytes". I assume this will tell the browser to use bytes when setting the Range header
    content-type: the content type of the file read. "audio/mpeg" for a mp3 file

    may God please guide me thru this shit code that is to ensue. I'm not a religious man but I might just be after this is done

*/