#include "arclib.h"

#include <stdio.h>
#include <stdarg.h>
#include <cstring>

//
//  ALStatus::ALStatus()
//
// ARGUMENTS:
//
//  None.
//
// RETURNS
//
//  Nothing, this is a constructor.
//
// DESCRIPTION
//
//  This is the only constructor for objects of class ALStatus.  It
//  initializes the detail length member to 129, which is a const and
//  won't change.  The initial status is AL_SUCCESS, and there is no
//  detail string to start with.
//
// REVISION HISTORY
//
//   May 26, 1994  1.0A  : First release
//

ALStatus::ALStatus() : miStatusDetailLength(129) {
  miStatus = AL_SUCCESS;
  mszStatusDetail = 0;
}

//
// ALStatus::~ALStatus()
//
// ARGUMENTS:
//
//  None, destructor.
//
// RETURNS
//
//  None, destructor.
//
// DESCRIPTION
//
//  The destructor has to free up any space allocated for the detailed
//  error status string.  That's all.
//
// REVISION HISTORY
//
//   May 26, 1994  1.0A  : First release
//

ALStatus::~ALStatus() {
  if (mszStatusDetail)
    delete[] mszStatusDetail;
}

//
// int ALStatus::SetError( int error, const char  *fmt, ... )
//
// ARGUMENTS:
//
//  error   :  The new error code to set the miStatus member to.  A value
//             less than 0 (AL_SUCCESS) will always be interpreted as
//             an error.
//
//  fmt     :  A sprintf style formatting string.  This is for the
//             message that is going to go into the status detail message.
//
//  ...     :  Any additional arguments needed by the formatting string.
//
//
// RETURNS
//
//  error, the error code that just got passed in.
//
// DESCRIPTION
//
//  I don't know why I did the status detail allocation the way it is
//  done here, it is really stupid. I should just allocate whatever space
//  is necessary after formatting the string.  This will probably be
//  fixed in 1.x.
//
//  This function is used to set the status of an object to an error state.
//  Normally this is done by sending an error code, along with a detailed
//  message explaining what went wrong and why.  Note that to clear
//  and error state, you can pass AL_SUCCESS for the error code and
//  0 for the format.  The object will look like it is healthy and happy
//  after that.
//
// REVISION HISTORY
//
//   May 26, 1994  1.0A  : First release
//

int ALStatus::SetError(int error, const char *fmt, ...) {
  char detail[256];
  va_list argptr;

  miStatus = error;
  if (fmt == 0) {
    if (mszStatusDetail)
      delete[] mszStatusDetail;
    mszStatusDetail = 0;
  } else {
    va_start(argptr, fmt);
    vsprintf(detail, fmt, argptr);
    va_end(argptr);
    if (mszStatusDetail == 0)
      mszStatusDetail = new char[miStatusDetailLength];
    if (mszStatusDetail) {
      strncpy(mszStatusDetail, detail, miStatusDetailLength - 1);
      mszStatusDetail[miStatusDetailLength - 1] = '\0';
    }
  }
  if (error) {
    fprintf(stderr, "ERROR CODE: %s\n", GetStatusString());
    if (mszStatusDetail) fprintf(stderr, " | %s\n", mszStatusDetail);
    fprintf(stderr, "ABORTING NOW\n");
    abort();
  }
  
  return error;
}

//
// const char * ALStatus::GetStatusString()
//
// ARGUMENTS:
//
//  None.
//
// RETURNS
//
//  A short ASCII translation of the current error code.
//
// DESCRIPTION
//
//  Rather than just printing an error code number, it is usually more
//  helpful to translate that number into ASCII text, so a user or
//  programmer can read the description.  This function is used to
//  do just that.  It translates the current error code into a short
//  ASCII text string.  Note that this is not the same as the detail
//  string, which is tailored for each specific occurrence of an error code.
//
// REVISION HISTORY
//
//   May 26, 1994  1.0A  : First release
//

const char *ALStatus::GetStatusString() {
  switch (miStatus) {
  case AL_SUCCESS:
    return "Success";
  case AL_END_OF_FILE:
    return "End of file";
  case AL_CANT_OPEN_BUFFER:
    return "Can't allocate buffer";
  case AL_CANT_CREATE_ENGINE:
    return "Can't create compression engine";
  case AL_CANT_CREATE_STORAGE_OBJECT:
    return "Can't create storage object";
  case AL_CANT_ALLOCATE_MEMORY:
    return "Memory allocation failure";
  case AL_RENAME_ERROR:
    return "Error renaming file";
  case AL_CANT_OPEN_FILE:
    return "Can't open file";
  case AL_SEEK_ERROR:
    return "Seek error";
  case AL_READ_ERROR:
    return "Read error";
  case AL_WRITE_ERROR:
    return "Write error";
  case AL_DELETE_ERROR:
    return "File deletion error";
  case AL_ILLEGAL_PARAMETER:
    return "Illegal parameter";
  case AL_INTERNAL_ERROR:
    return "Internal error";
  case AL_USER_ABORT:
    return "User abort";
  case AL_SERVER_NOT_PRESENT:
    return "Server not present";
  case AL_COMPRESSION_TYPE_MISMATCH:
    return "Mismatch in compression type";
  case AL_NEED_LENGTH:
    return "Missing length parameter";
  case AL_CRC_ERROR:
    return "CRC Error";
  case AL_COMPARE_ERROR:
    return "Comparison error";
  case AL_UNKNOWN_COMPRESSION_TYPE:
    return "Unknown compression type";
  case AL_UNKNOWN_STORAGE_OBJECT:
    return "Unknown type of storage object";
  case AL_INVALID_ARCHIVE:
    return "Invalid archive";
  case AL_LOGIC_ERROR:
    return "Logic error";
  case AL_BACKUP_FAILURE:
    return "Could not create backup";
  case AL_GETSEL_ERROR:
    return "Error getting selections from list box";
  case AL_DUPLICATE_ENTRY:
    return "Duplicate entry";
  default:
    return "Unknown error";
  }
}

//
// const char * ALStatus::GetStatusDetail() const
//
// ARGUMENTS:
//
//  None.
//
// RETURNS
//
//  Guaranteed to return a valid character string.
//
// DESCRIPTION
//
//  Whenever we set the error code for an object in ArchiveLib, we
//  call ALStatus::SetError().  At the same time that we set the
//  error code of the object to a non-zero value, we supply a formatted
//  string providing some detail about when and where the error
//  took place, maybe even including some other information provided by the
//  O/S.  That information is stored in the detail string, which is a
//  private data member.  This function provides the ability to get at
//  that detail string.
//
// REVISION HISTORY
//
//   May 26, 1994  1.0A  : First release
//

const char *ALStatus::GetStatusDetail() const {
  if (mszStatusDetail)
    return mszStatusDetail;
  else if (miStatus == AL_SUCCESS)
    return "No errors";
  else
    return "Unable to allocate memory for error detail message";
}

//
// ALStatus & ALStatus::operator = ( ALStatus &rhs )
//
// ARGUMENTS:
//
//  rhs  : Another ALStatus object that I want to copy into this object.
//
// RETURNS
//
//  A reference to this.
//
// DESCRIPTION
//
//  Somewhere in ArchiveLib I want to be able to copy one status
//  into another.  This function does just that.  It has to allocate
//  new space to make a copy of the detail string, and be sure to
//  free up any old space, and all that.
//
// REVISION HISTORY
//
//   May 26, 1994  1.0A  : First release
//

ALStatus &ALStatus::operator=(ALStatus &rhs) {
  if (rhs.mszStatusDetail == 0) {
    if (mszStatusDetail) {
      delete[] mszStatusDetail;
      mszStatusDetail = 0;
    }
  } else {
    if (mszStatusDetail == 0)
      mszStatusDetail = new char[miStatusDetailLength];
    if (mszStatusDetail)
      strcpy(mszStatusDetail, rhs.mszStatusDetail);
  }
  miStatus = rhs.miStatus;
  return *this;
}

SimpleStatus ALStatus::copyToSimple() {
  const char *data = GetStatusDetail();
  size_t len = strlen(data) + 1;
  u_int8_t *raw_data = NULL;
  if (len > 1) {
    raw_data = (u_int8_t *)calloc(len, sizeof(char));
    memcpy(raw_data, data, len);
  }
  return SimpleStatus{miStatus, raw_data, len};
}