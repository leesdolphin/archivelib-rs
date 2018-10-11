/*
Copyright 1990-2008 Light Infocon Tecnologia S/A

Este arquivo é parte do programa LightBase - Banco de Dados Textual Documental

O LightBase é um software livre; você pode redistribui-lo e/ou modifica-lo dentro
dos termos da Licença Pública Geral GNU como publicada pela Fundação do Software
Livre (FSF); na versão 2 da Licença.

Este programa é distribuído na esperança que possa ser útil, mas SEM NENHUMA
GARANTIA; sem uma garantia implícita de ADEQUAÇÃO a qualquer MERCADO ou APLICAÇÃO
EM PARTICULAR. Veja a Licença Pública Geral GNU para maiores detalhes.

Você deve ter recebido uma cópia da Licença Pública Geral GNU versao 2, sob o
título "LICENCA.txt", junto com este programa, se não, escreva para a Fundação do
Software Livre(FSF) Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA
*/


#ifndef _WINMON_H
#define _WINMON_H

#include "arclib.h"

#if defined( __cplusplus )

/*
 * class ALWindowsMessage
 *
 * DESCRIPTION
 *
 * This class is used to provide user feedback when operating under
 * windows.  It can be constructed to send messages to windows from
 * YieldTime(), allowing you to easily update progress bars, text
 * boxes, or whatever.
 *
 * DATA MEMBERS
 *
 *  mhMessageWindowHandle   : The handle of the window that is going to
 *                            get the text messages generated by the
 *                            ArchiveOperation() procedure.  If this
 *                            member is set to 0, no messages are sent.
 *
 *  mhNumberWindowHandle    : The handle of the window that is going to
 *                            get either the byte count or the percent
 *                            complete figure.  If miMessage is 0, it
 *                            is formatted as ASCII and sent using a
 *                            SetWindowText() call.  O/W, it is sent
 *                            using SendMessage(), in Lparam and Wparam.
 *
 *  mMessageType            : AL_SEND_BYTE_COUNT or AL_SEND_RATIO.
 *
 *  miMessage               : The message that gets sent with with the
 *                            byte count or ratio.
 *
 * MEMBER FUNCTIONS
 *
 *  ALWindowsMessage()      : The one and only constructor.
 *  ~ALWindowsMessage()     : The destructor.
 *  operator new()          : The memory allocation operator, only used
 *                            when the library is in a DLL.
 *  Progress()              : The virtual function that gets called to
 *                            update progress through the file/job.
 *  ArchiveOperation()      : The virtual function that gets called
 *                            at key points in the archiving process.
 *
 * REVISION HISTORY
 *
 *  May 26, 1994  1.0A  : First release
 *
 */


class AL_CLASS_TYPE ALWindowsMessage : public ALMonitor {
/*
 * Constructors, destructors, and friends
 */
    public :
        AL_PROTO ALWindowsMessage( ALMonitorType monitor_type,
                                   HWND progress_text_window,
                                   ALWindowsMessageType message_type,
                                   HWND progress_number_window,
                                   UINT windows_message = 0 );
        virtual AL_PROTO ~ALWindowsMessage();
#if defined( AL_USING_DLL ) || defined( AL_BUILDING_DLL )
        void AL_DLL_FAR * AL_PROTO operator new( size_t size );
#endif
    protected :
        virtual void AL_PROTO Progress( long mlObjectSoFar,
                                        ALStorage AL_DLL_FAR & object );
        virtual void AL_PROTO
        ArchiveOperation( ALArchiveOperation operation,
                          ALArchiveBase AL_DLL_FAR *archive,
                          ALEntry AL_DLL_FAR *job );
/*
 * Data members
 */
    protected :
        HWND mhMessageWindowHandle;
        HWND mhNumberWindowHandle;
        ALWindowsMessageType mMessageType;
        int miMessage;
    public :
        AL_CLASS_TAG( _ALWindowsMessageTag );
};

#endif /* #if defined( __cplusplus ) */

#endif /* #ifdef _WINMON_H           */
