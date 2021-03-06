use std::ffi::{CString, CStr};
use libc::c_char;

use sys::scancode as ll;

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Scancode {
    A                  = ll::SDL_SCANCODE_A as i32,
    B                  = ll::SDL_SCANCODE_B as i32,
    C                  = ll::SDL_SCANCODE_C as i32,
    D                  = ll::SDL_SCANCODE_D as i32,
    E                  = ll::SDL_SCANCODE_E as i32,
    F                  = ll::SDL_SCANCODE_F as i32,
    G                  = ll::SDL_SCANCODE_G as i32,
    H                  = ll::SDL_SCANCODE_H as i32,
    I                  = ll::SDL_SCANCODE_I as i32,
    J                  = ll::SDL_SCANCODE_J as i32,
    K                  = ll::SDL_SCANCODE_K as i32,
    L                  = ll::SDL_SCANCODE_L as i32,
    M                  = ll::SDL_SCANCODE_M as i32,
    N                  = ll::SDL_SCANCODE_N as i32,
    O                  = ll::SDL_SCANCODE_O as i32,
    P                  = ll::SDL_SCANCODE_P as i32,
    Q                  = ll::SDL_SCANCODE_Q as i32,
    R                  = ll::SDL_SCANCODE_R as i32,
    S                  = ll::SDL_SCANCODE_S as i32,
    T                  = ll::SDL_SCANCODE_T as i32,
    U                  = ll::SDL_SCANCODE_U as i32,
    V                  = ll::SDL_SCANCODE_V as i32,
    W                  = ll::SDL_SCANCODE_W as i32,
    X                  = ll::SDL_SCANCODE_X as i32,
    Y                  = ll::SDL_SCANCODE_Y as i32,
    Z                  = ll::SDL_SCANCODE_Z as i32,
    Num1               = ll::SDL_SCANCODE_1 as i32,
    Num2               = ll::SDL_SCANCODE_2 as i32,
    Num3               = ll::SDL_SCANCODE_3 as i32,
    Num4               = ll::SDL_SCANCODE_4 as i32,
    Num5               = ll::SDL_SCANCODE_5 as i32,
    Num6               = ll::SDL_SCANCODE_6 as i32,
    Num7               = ll::SDL_SCANCODE_7 as i32,
    Num8               = ll::SDL_SCANCODE_8 as i32,
    Num9               = ll::SDL_SCANCODE_9 as i32,
    Num0               = ll::SDL_SCANCODE_0 as i32,
    Return             = ll::SDL_SCANCODE_RETURN as i32,
    Escape             = ll::SDL_SCANCODE_ESCAPE as i32,
    Backspace          = ll::SDL_SCANCODE_BACKSPACE as i32,
    Tab                = ll::SDL_SCANCODE_TAB as i32,
    Space              = ll::SDL_SCANCODE_SPACE as i32,
    Minus              = ll::SDL_SCANCODE_MINUS as i32,
    Equals             = ll::SDL_SCANCODE_EQUALS as i32,
    LeftBracket        = ll::SDL_SCANCODE_LEFTBRACKET as i32,
    RightBracket       = ll::SDL_SCANCODE_RIGHTBRACKET as i32,
    Backslash          = ll::SDL_SCANCODE_BACKSLASH as i32,
    NonUsHash          = ll::SDL_SCANCODE_NONUSHASH as i32,
    Semicolon          = ll::SDL_SCANCODE_SEMICOLON as i32,
    Apostrophe         = ll::SDL_SCANCODE_APOSTROPHE as i32,
    Grave              = ll::SDL_SCANCODE_GRAVE as i32,
    Comma              = ll::SDL_SCANCODE_COMMA as i32,
    Period             = ll::SDL_SCANCODE_PERIOD as i32,
    Slash              = ll::SDL_SCANCODE_SLASH as i32,
    CapsLock           = ll::SDL_SCANCODE_CAPSLOCK as i32,
    F1                 = ll::SDL_SCANCODE_F1 as i32,
    F2                 = ll::SDL_SCANCODE_F2 as i32,
    F3                 = ll::SDL_SCANCODE_F3 as i32,
    F4                 = ll::SDL_SCANCODE_F4 as i32,
    F5                 = ll::SDL_SCANCODE_F5 as i32,
    F6                 = ll::SDL_SCANCODE_F6 as i32,
    F7                 = ll::SDL_SCANCODE_F7 as i32,
    F8                 = ll::SDL_SCANCODE_F8 as i32,
    F9                 = ll::SDL_SCANCODE_F9 as i32,
    F10                = ll::SDL_SCANCODE_F10 as i32,
    F11                = ll::SDL_SCANCODE_F11 as i32,
    F12                = ll::SDL_SCANCODE_F12 as i32,
    PrintScreen        = ll::SDL_SCANCODE_PRINTSCREEN as i32,
    ScrollLock         = ll::SDL_SCANCODE_SCROLLLOCK as i32,
    Pause              = ll::SDL_SCANCODE_PAUSE as i32,
    Insert             = ll::SDL_SCANCODE_INSERT as i32,
    Home               = ll::SDL_SCANCODE_HOME as i32,
    PageUp             = ll::SDL_SCANCODE_PAGEUP as i32,
    Delete             = ll::SDL_SCANCODE_DELETE as i32,
    End                = ll::SDL_SCANCODE_END as i32,
    PageDown           = ll::SDL_SCANCODE_PAGEDOWN as i32,
    Right              = ll::SDL_SCANCODE_RIGHT as i32,
    Left               = ll::SDL_SCANCODE_LEFT as i32,
    Down               = ll::SDL_SCANCODE_DOWN as i32,
    Up                 = ll::SDL_SCANCODE_UP as i32,
    NumLockClear       = ll::SDL_SCANCODE_NUMLOCKCLEAR as i32,
    KpDivide           = ll::SDL_SCANCODE_KP_DIVIDE as i32,
    KpMultiply         = ll::SDL_SCANCODE_KP_MULTIPLY as i32,
    KpMinus            = ll::SDL_SCANCODE_KP_MINUS as i32,
    KpPlus             = ll::SDL_SCANCODE_KP_PLUS as i32,
    KpEnter            = ll::SDL_SCANCODE_KP_ENTER as i32,
    Kp1                = ll::SDL_SCANCODE_KP_1 as i32,
    Kp2                = ll::SDL_SCANCODE_KP_2 as i32,
    Kp3                = ll::SDL_SCANCODE_KP_3 as i32,
    Kp4                = ll::SDL_SCANCODE_KP_4 as i32,
    Kp5                = ll::SDL_SCANCODE_KP_5 as i32,
    Kp6                = ll::SDL_SCANCODE_KP_6 as i32,
    Kp7                = ll::SDL_SCANCODE_KP_7 as i32,
    Kp8                = ll::SDL_SCANCODE_KP_8 as i32,
    Kp9                = ll::SDL_SCANCODE_KP_9 as i32,
    Kp0                = ll::SDL_SCANCODE_KP_0 as i32,
    KpPeriod           = ll::SDL_SCANCODE_KP_PERIOD as i32,
    NonUsBackslash     = ll::SDL_SCANCODE_NONUSBACKSLASH as i32,
    Application        = ll::SDL_SCANCODE_APPLICATION as i32,
    Power              = ll::SDL_SCANCODE_POWER as i32,
    KpEquals           = ll::SDL_SCANCODE_KP_EQUALS as i32,
    F13                = ll::SDL_SCANCODE_F13 as i32,
    F14                = ll::SDL_SCANCODE_F14 as i32,
    F15                = ll::SDL_SCANCODE_F15 as i32,
    F16                = ll::SDL_SCANCODE_F16 as i32,
    F17                = ll::SDL_SCANCODE_F17 as i32,
    F18                = ll::SDL_SCANCODE_F18 as i32,
    F19                = ll::SDL_SCANCODE_F19 as i32,
    F20                = ll::SDL_SCANCODE_F20 as i32,
    F21                = ll::SDL_SCANCODE_F21 as i32,
    F22                = ll::SDL_SCANCODE_F22 as i32,
    F23                = ll::SDL_SCANCODE_F23 as i32,
    F24                = ll::SDL_SCANCODE_F24 as i32,
    Execute            = ll::SDL_SCANCODE_EXECUTE as i32,
    Help               = ll::SDL_SCANCODE_HELP as i32,
    Menu               = ll::SDL_SCANCODE_MENU as i32,
    Select             = ll::SDL_SCANCODE_SELECT as i32,
    Stop               = ll::SDL_SCANCODE_STOP as i32,
    Again              = ll::SDL_SCANCODE_AGAIN as i32,
    Undo               = ll::SDL_SCANCODE_UNDO as i32,
    Cut                = ll::SDL_SCANCODE_CUT as i32,
    Copy               = ll::SDL_SCANCODE_COPY as i32,
    Paste              = ll::SDL_SCANCODE_PASTE as i32,
    Find               = ll::SDL_SCANCODE_FIND as i32,
    Mute               = ll::SDL_SCANCODE_MUTE as i32,
    VolumeUp           = ll::SDL_SCANCODE_VOLUMEUP as i32,
    VolumeDown         = ll::SDL_SCANCODE_VOLUMEDOWN as i32,
    KpComma            = ll::SDL_SCANCODE_KP_COMMA as i32,
    KpEqualsAS400      = ll::SDL_SCANCODE_KP_EQUALSAS400 as i32,
    International1     = ll::SDL_SCANCODE_INTERNATIONAL1 as i32,
    International2     = ll::SDL_SCANCODE_INTERNATIONAL2 as i32,
    International3     = ll::SDL_SCANCODE_INTERNATIONAL3 as i32,
    International4     = ll::SDL_SCANCODE_INTERNATIONAL4 as i32,
    International5     = ll::SDL_SCANCODE_INTERNATIONAL5 as i32,
    International6     = ll::SDL_SCANCODE_INTERNATIONAL6 as i32,
    International7     = ll::SDL_SCANCODE_INTERNATIONAL7 as i32,
    International8     = ll::SDL_SCANCODE_INTERNATIONAL8 as i32,
    International9     = ll::SDL_SCANCODE_INTERNATIONAL9 as i32,
    Lang1              = ll::SDL_SCANCODE_LANG1 as i32,
    Lang2              = ll::SDL_SCANCODE_LANG2 as i32,
    Lang3              = ll::SDL_SCANCODE_LANG3 as i32,
    Lang4              = ll::SDL_SCANCODE_LANG4 as i32,
    Lang5              = ll::SDL_SCANCODE_LANG5 as i32,
    Lang6              = ll::SDL_SCANCODE_LANG6 as i32,
    Lang7              = ll::SDL_SCANCODE_LANG7 as i32,
    Lang8              = ll::SDL_SCANCODE_LANG8 as i32,
    Lang9              = ll::SDL_SCANCODE_LANG9 as i32,
    AltErase           = ll::SDL_SCANCODE_ALTERASE as i32,
    SysReq             = ll::SDL_SCANCODE_SYSREQ as i32,
    Cancel             = ll::SDL_SCANCODE_CANCEL as i32,
    Clear              = ll::SDL_SCANCODE_CLEAR as i32,
    Prior              = ll::SDL_SCANCODE_PRIOR as i32,
    Return2            = ll::SDL_SCANCODE_RETURN2 as i32,
    Separator          = ll::SDL_SCANCODE_SEPARATOR as i32,
    Out                = ll::SDL_SCANCODE_OUT as i32,
    Oper               = ll::SDL_SCANCODE_OPER as i32,
    ClearAgain         = ll::SDL_SCANCODE_CLEARAGAIN as i32,
    CrSel              = ll::SDL_SCANCODE_CRSEL as i32,
    ExSel              = ll::SDL_SCANCODE_EXSEL as i32,
    Kp00               = ll::SDL_SCANCODE_KP_00 as i32,
    Kp000              = ll::SDL_SCANCODE_KP_000 as i32,
    ThousandsSeparator = ll::SDL_SCANCODE_THOUSANDSSEPARATOR as i32,
    DecimalSeparator   = ll::SDL_SCANCODE_DECIMALSEPARATOR as i32,
    CurrencyUnit       = ll::SDL_SCANCODE_CURRENCYUNIT as i32,
    CurrencySubUnit    = ll::SDL_SCANCODE_CURRENCYSUBUNIT as i32,
    KpLeftParen        = ll::SDL_SCANCODE_KP_LEFTPAREN as i32,
    KpRightParen       = ll::SDL_SCANCODE_KP_RIGHTPAREN as i32,
    KpLeftBrace        = ll::SDL_SCANCODE_KP_LEFTBRACE as i32,
    KpRightBrace       = ll::SDL_SCANCODE_KP_RIGHTBRACE as i32,
    KpTab              = ll::SDL_SCANCODE_KP_TAB as i32,
    KpBackspace        = ll::SDL_SCANCODE_KP_BACKSPACE as i32,
    KpA                = ll::SDL_SCANCODE_KP_A as i32,
    KpB                = ll::SDL_SCANCODE_KP_B as i32,
    KpC                = ll::SDL_SCANCODE_KP_C as i32,
    KpD                = ll::SDL_SCANCODE_KP_D as i32,
    KpE                = ll::SDL_SCANCODE_KP_E as i32,
    KpF                = ll::SDL_SCANCODE_KP_F as i32,
    KpXor              = ll::SDL_SCANCODE_KP_XOR as i32,
    KpPower            = ll::SDL_SCANCODE_KP_POWER as i32,
    KpPercent          = ll::SDL_SCANCODE_KP_PERCENT as i32,
    KpLess             = ll::SDL_SCANCODE_KP_LESS as i32,
    KpGreater          = ll::SDL_SCANCODE_KP_GREATER as i32,
    KpAmpersand        = ll::SDL_SCANCODE_KP_AMPERSAND as i32,
    KpDblAmpersand     = ll::SDL_SCANCODE_KP_DBLAMPERSAND as i32,
    KpVerticalBar      = ll::SDL_SCANCODE_KP_VERTICALBAR as i32,
    KpDblVerticalBar   = ll::SDL_SCANCODE_KP_DBLVERTICALBAR as i32,
    KpColon            = ll::SDL_SCANCODE_KP_COLON as i32,
    KpHash             = ll::SDL_SCANCODE_KP_HASH as i32,
    KpSpace            = ll::SDL_SCANCODE_KP_SPACE as i32,
    KpAt               = ll::SDL_SCANCODE_KP_AT as i32,
    KpExclam           = ll::SDL_SCANCODE_KP_EXCLAM as i32,
    KpMemStore         = ll::SDL_SCANCODE_KP_MEMSTORE as i32,
    KpMemRecall        = ll::SDL_SCANCODE_KP_MEMRECALL as i32,
    KpMemClear         = ll::SDL_SCANCODE_KP_MEMCLEAR as i32,
    KpMemAdd           = ll::SDL_SCANCODE_KP_MEMADD as i32,
    KpMemSubtract      = ll::SDL_SCANCODE_KP_MEMSUBTRACT as i32,
    KpMemMultiply      = ll::SDL_SCANCODE_KP_MEMMULTIPLY as i32,
    KpMemDivide        = ll::SDL_SCANCODE_KP_MEMDIVIDE as i32,
    KpPlusMinus        = ll::SDL_SCANCODE_KP_PLUSMINUS as i32,
    KpClear            = ll::SDL_SCANCODE_KP_CLEAR as i32,
    KpClearEntry       = ll::SDL_SCANCODE_KP_CLEARENTRY as i32,
    KpBinary           = ll::SDL_SCANCODE_KP_BINARY as i32,
    KpOctal            = ll::SDL_SCANCODE_KP_OCTAL as i32,
    KpDecimal          = ll::SDL_SCANCODE_KP_DECIMAL as i32,
    KpHexadecimal      = ll::SDL_SCANCODE_KP_HEXADECIMAL as i32,
    LCtrl              = ll::SDL_SCANCODE_LCTRL as i32,
    LShift             = ll::SDL_SCANCODE_LSHIFT as i32,
    LAlt               = ll::SDL_SCANCODE_LALT as i32,
    LGui               = ll::SDL_SCANCODE_LGUI as i32,
    RCtrl              = ll::SDL_SCANCODE_RCTRL as i32,
    RShift             = ll::SDL_SCANCODE_RSHIFT as i32,
    RAlt               = ll::SDL_SCANCODE_RALT as i32,
    RGui               = ll::SDL_SCANCODE_RGUI as i32,
    Mode               = ll::SDL_SCANCODE_MODE as i32,
    AudioNext          = ll::SDL_SCANCODE_AUDIONEXT as i32,
    AudioPrev          = ll::SDL_SCANCODE_AUDIOPREV as i32,
    AudioStop          = ll::SDL_SCANCODE_AUDIOSTOP as i32,
    AudioPlay          = ll::SDL_SCANCODE_AUDIOPLAY as i32,
    AudioMute          = ll::SDL_SCANCODE_AUDIOMUTE as i32,
    MediaSelect        = ll::SDL_SCANCODE_MEDIASELECT as i32,
    Www                = ll::SDL_SCANCODE_WWW as i32,
    Mail               = ll::SDL_SCANCODE_MAIL as i32,
    Calculator         = ll::SDL_SCANCODE_CALCULATOR as i32,
    Computer           = ll::SDL_SCANCODE_COMPUTER as i32,
    AcSearch           = ll::SDL_SCANCODE_AC_SEARCH as i32,
    AcHome             = ll::SDL_SCANCODE_AC_HOME as i32,
    AcBack             = ll::SDL_SCANCODE_AC_BACK as i32,
    AcForward          = ll::SDL_SCANCODE_AC_FORWARD as i32,
    AcStop             = ll::SDL_SCANCODE_AC_STOP as i32,
    AcRefresh          = ll::SDL_SCANCODE_AC_REFRESH as i32,
    AcBookmarks        = ll::SDL_SCANCODE_AC_BOOKMARKS as i32,
    BrightnessDown     = ll::SDL_SCANCODE_BRIGHTNESSDOWN as i32,
    BrightnessUp       = ll::SDL_SCANCODE_BRIGHTNESSUP as i32,
    DisplaySwitch      = ll::SDL_SCANCODE_DISPLAYSWITCH as i32,
    KbdIllumToggle     = ll::SDL_SCANCODE_KBDILLUMTOGGLE as i32,
    KbdIllumDown       = ll::SDL_SCANCODE_KBDILLUMDOWN as i32,
    KbdIllumUp         = ll::SDL_SCANCODE_KBDILLUMUP as i32,
    Eject              = ll::SDL_SCANCODE_EJECT as i32,
    Sleep              = ll::SDL_SCANCODE_SLEEP as i32,
    App1               = ll::SDL_SCANCODE_APP1 as i32,
    App2               = ll::SDL_SCANCODE_APP2 as i32,
    Num                = ll::SDL_NUM_SCANCODES as i32,
}

impl Scancode {
    pub fn from_i32(n: i32) -> Option<Scancode> {
        use self::Scancode::*;

        Some( match n as ll::SDL_Scancode {
            ll::SDL_SCANCODE_UNKNOWN            => return None,
            ll::SDL_SCANCODE_A                  => A,
            ll::SDL_SCANCODE_B                  => B,
            ll::SDL_SCANCODE_C                  => C,
            ll::SDL_SCANCODE_D                  => D,
            ll::SDL_SCANCODE_E                  => E,
            ll::SDL_SCANCODE_F                  => F,
            ll::SDL_SCANCODE_G                  => G,
            ll::SDL_SCANCODE_H                  => H,
            ll::SDL_SCANCODE_I                  => I,
            ll::SDL_SCANCODE_J                  => J,
            ll::SDL_SCANCODE_K                  => K,
            ll::SDL_SCANCODE_L                  => L,
            ll::SDL_SCANCODE_M                  => M,
            ll::SDL_SCANCODE_N                  => N,
            ll::SDL_SCANCODE_O                  => O,
            ll::SDL_SCANCODE_P                  => P,
            ll::SDL_SCANCODE_Q                  => Q,
            ll::SDL_SCANCODE_R                  => R,
            ll::SDL_SCANCODE_S                  => S,
            ll::SDL_SCANCODE_T                  => T,
            ll::SDL_SCANCODE_U                  => U,
            ll::SDL_SCANCODE_V                  => V,
            ll::SDL_SCANCODE_W                  => W,
            ll::SDL_SCANCODE_X                  => X,
            ll::SDL_SCANCODE_Y                  => Y,
            ll::SDL_SCANCODE_Z                  => Z,
            ll::SDL_SCANCODE_1                  => Num1,
            ll::SDL_SCANCODE_2                  => Num2,
            ll::SDL_SCANCODE_3                  => Num3,
            ll::SDL_SCANCODE_4                  => Num4,
            ll::SDL_SCANCODE_5                  => Num5,
            ll::SDL_SCANCODE_6                  => Num6,
            ll::SDL_SCANCODE_7                  => Num7,
            ll::SDL_SCANCODE_8                  => Num8,
            ll::SDL_SCANCODE_9                  => Num9,
            ll::SDL_SCANCODE_0                  => Num0,
            ll::SDL_SCANCODE_RETURN             => Return,
            ll::SDL_SCANCODE_ESCAPE             => Escape,
            ll::SDL_SCANCODE_BACKSPACE          => Backspace,
            ll::SDL_SCANCODE_TAB                => Tab,
            ll::SDL_SCANCODE_SPACE              => Space,
            ll::SDL_SCANCODE_MINUS              => Minus,
            ll::SDL_SCANCODE_EQUALS             => Equals,
            ll::SDL_SCANCODE_LEFTBRACKET        => LeftBracket,
            ll::SDL_SCANCODE_RIGHTBRACKET       => RightBracket,
            ll::SDL_SCANCODE_BACKSLASH          => Backslash,
            ll::SDL_SCANCODE_NONUSHASH          => NonUsHash,
            ll::SDL_SCANCODE_SEMICOLON          => Semicolon,
            ll::SDL_SCANCODE_APOSTROPHE         => Apostrophe,
            ll::SDL_SCANCODE_GRAVE              => Grave,
            ll::SDL_SCANCODE_COMMA              => Comma,
            ll::SDL_SCANCODE_PERIOD             => Period,
            ll::SDL_SCANCODE_SLASH              => Slash,
            ll::SDL_SCANCODE_CAPSLOCK           => CapsLock,
            ll::SDL_SCANCODE_F1                 => F1,
            ll::SDL_SCANCODE_F2                 => F2,
            ll::SDL_SCANCODE_F3                 => F3,
            ll::SDL_SCANCODE_F4                 => F4,
            ll::SDL_SCANCODE_F5                 => F5,
            ll::SDL_SCANCODE_F6                 => F6,
            ll::SDL_SCANCODE_F7                 => F7,
            ll::SDL_SCANCODE_F8                 => F8,
            ll::SDL_SCANCODE_F9                 => F9,
            ll::SDL_SCANCODE_F10                => F10,
            ll::SDL_SCANCODE_F11                => F11,
            ll::SDL_SCANCODE_F12                => F12,
            ll::SDL_SCANCODE_PRINTSCREEN        => PrintScreen,
            ll::SDL_SCANCODE_SCROLLLOCK         => ScrollLock,
            ll::SDL_SCANCODE_PAUSE              => Pause,
            ll::SDL_SCANCODE_INSERT             => Insert,
            ll::SDL_SCANCODE_HOME               => Home,
            ll::SDL_SCANCODE_PAGEUP             => PageUp,
            ll::SDL_SCANCODE_DELETE             => Delete,
            ll::SDL_SCANCODE_END                => End,
            ll::SDL_SCANCODE_PAGEDOWN           => PageDown,
            ll::SDL_SCANCODE_RIGHT              => Right,
            ll::SDL_SCANCODE_LEFT               => Left,
            ll::SDL_SCANCODE_DOWN               => Down,
            ll::SDL_SCANCODE_UP                 => Up,
            ll::SDL_SCANCODE_NUMLOCKCLEAR       => NumLockClear,
            ll::SDL_SCANCODE_KP_DIVIDE          => KpDivide,
            ll::SDL_SCANCODE_KP_MULTIPLY        => KpMultiply,
            ll::SDL_SCANCODE_KP_MINUS           => KpMinus,
            ll::SDL_SCANCODE_KP_PLUS            => KpPlus,
            ll::SDL_SCANCODE_KP_ENTER           => KpEnter,
            ll::SDL_SCANCODE_KP_1               => Kp1,
            ll::SDL_SCANCODE_KP_2               => Kp2,
            ll::SDL_SCANCODE_KP_3               => Kp3,
            ll::SDL_SCANCODE_KP_4               => Kp4,
            ll::SDL_SCANCODE_KP_5               => Kp5,
            ll::SDL_SCANCODE_KP_6               => Kp6,
            ll::SDL_SCANCODE_KP_7               => Kp7,
            ll::SDL_SCANCODE_KP_8               => Kp8,
            ll::SDL_SCANCODE_KP_9               => Kp9,
            ll::SDL_SCANCODE_KP_0               => Kp0,
            ll::SDL_SCANCODE_KP_PERIOD          => KpPeriod,
            ll::SDL_SCANCODE_NONUSBACKSLASH     => NonUsBackslash,
            ll::SDL_SCANCODE_APPLICATION        => Application,
            ll::SDL_SCANCODE_POWER              => Power,
            ll::SDL_SCANCODE_KP_EQUALS          => KpEquals,
            ll::SDL_SCANCODE_F13                => F13,
            ll::SDL_SCANCODE_F14                => F14,
            ll::SDL_SCANCODE_F15                => F15,
            ll::SDL_SCANCODE_F16                => F16,
            ll::SDL_SCANCODE_F17                => F17,
            ll::SDL_SCANCODE_F18                => F18,
            ll::SDL_SCANCODE_F19                => F19,
            ll::SDL_SCANCODE_F20                => F20,
            ll::SDL_SCANCODE_F21                => F21,
            ll::SDL_SCANCODE_F22                => F22,
            ll::SDL_SCANCODE_F23                => F23,
            ll::SDL_SCANCODE_F24                => F24,
            ll::SDL_SCANCODE_EXECUTE            => Execute,
            ll::SDL_SCANCODE_HELP               => Help,
            ll::SDL_SCANCODE_MENU               => Menu,
            ll::SDL_SCANCODE_SELECT             => Select,
            ll::SDL_SCANCODE_STOP               => Stop,
            ll::SDL_SCANCODE_AGAIN              => Again,
            ll::SDL_SCANCODE_UNDO               => Undo,
            ll::SDL_SCANCODE_CUT                => Cut,
            ll::SDL_SCANCODE_COPY               => Copy,
            ll::SDL_SCANCODE_PASTE              => Paste,
            ll::SDL_SCANCODE_FIND               => Find,
            ll::SDL_SCANCODE_MUTE               => Mute,
            ll::SDL_SCANCODE_VOLUMEUP           => VolumeUp,
            ll::SDL_SCANCODE_VOLUMEDOWN         => VolumeDown,
            ll::SDL_SCANCODE_KP_COMMA           => KpComma,
            ll::SDL_SCANCODE_KP_EQUALSAS400     => KpEqualsAS400,
            ll::SDL_SCANCODE_INTERNATIONAL1     => International1,
            ll::SDL_SCANCODE_INTERNATIONAL2     => International2,
            ll::SDL_SCANCODE_INTERNATIONAL3     => International3,
            ll::SDL_SCANCODE_INTERNATIONAL4     => International4,
            ll::SDL_SCANCODE_INTERNATIONAL5     => International5,
            ll::SDL_SCANCODE_INTERNATIONAL6     => International6,
            ll::SDL_SCANCODE_INTERNATIONAL7     => International7,
            ll::SDL_SCANCODE_INTERNATIONAL8     => International8,
            ll::SDL_SCANCODE_INTERNATIONAL9     => International9,
            ll::SDL_SCANCODE_LANG1              => Lang1,
            ll::SDL_SCANCODE_LANG2              => Lang2,
            ll::SDL_SCANCODE_LANG3              => Lang3,
            ll::SDL_SCANCODE_LANG4              => Lang4,
            ll::SDL_SCANCODE_LANG5              => Lang5,
            ll::SDL_SCANCODE_LANG6              => Lang6,
            ll::SDL_SCANCODE_LANG7              => Lang7,
            ll::SDL_SCANCODE_LANG8              => Lang8,
            ll::SDL_SCANCODE_LANG9              => Lang9,
            ll::SDL_SCANCODE_ALTERASE           => AltErase,
            ll::SDL_SCANCODE_SYSREQ             => SysReq,
            ll::SDL_SCANCODE_CANCEL             => Cancel,
            ll::SDL_SCANCODE_CLEAR              => Clear,
            ll::SDL_SCANCODE_PRIOR              => Prior,
            ll::SDL_SCANCODE_RETURN2            => Return2,
            ll::SDL_SCANCODE_SEPARATOR          => Separator,
            ll::SDL_SCANCODE_OUT                => Out,
            ll::SDL_SCANCODE_OPER               => Oper,
            ll::SDL_SCANCODE_CLEARAGAIN         => ClearAgain,
            ll::SDL_SCANCODE_CRSEL              => CrSel,
            ll::SDL_SCANCODE_EXSEL              => ExSel,
            ll::SDL_SCANCODE_KP_00              => Kp00,
            ll::SDL_SCANCODE_KP_000             => Kp000,
            ll::SDL_SCANCODE_THOUSANDSSEPARATOR => ThousandsSeparator,
            ll::SDL_SCANCODE_DECIMALSEPARATOR   => DecimalSeparator,
            ll::SDL_SCANCODE_CURRENCYUNIT       => CurrencyUnit,
            ll::SDL_SCANCODE_CURRENCYSUBUNIT    => CurrencySubUnit,
            ll::SDL_SCANCODE_KP_LEFTPAREN       => KpLeftParen,
            ll::SDL_SCANCODE_KP_RIGHTPAREN      => KpRightParen,
            ll::SDL_SCANCODE_KP_LEFTBRACE       => KpLeftBrace,
            ll::SDL_SCANCODE_KP_RIGHTBRACE      => KpRightBrace,
            ll::SDL_SCANCODE_KP_TAB             => KpTab,
            ll::SDL_SCANCODE_KP_BACKSPACE       => KpBackspace,
            ll::SDL_SCANCODE_KP_A               => KpA,
            ll::SDL_SCANCODE_KP_B               => KpB,
            ll::SDL_SCANCODE_KP_C               => KpC,
            ll::SDL_SCANCODE_KP_D               => KpD,
            ll::SDL_SCANCODE_KP_E               => KpE,
            ll::SDL_SCANCODE_KP_F               => KpF,
            ll::SDL_SCANCODE_KP_XOR             => KpXor,
            ll::SDL_SCANCODE_KP_POWER           => KpPower,
            ll::SDL_SCANCODE_KP_PERCENT         => KpPercent,
            ll::SDL_SCANCODE_KP_LESS            => KpLess,
            ll::SDL_SCANCODE_KP_GREATER         => KpGreater,
            ll::SDL_SCANCODE_KP_AMPERSAND       => KpAmpersand,
            ll::SDL_SCANCODE_KP_DBLAMPERSAND    => KpDblAmpersand,
            ll::SDL_SCANCODE_KP_VERTICALBAR     => KpVerticalBar,
            ll::SDL_SCANCODE_KP_DBLVERTICALBAR  => KpDblVerticalBar,
            ll::SDL_SCANCODE_KP_COLON           => KpColon,
            ll::SDL_SCANCODE_KP_HASH            => KpHash,
            ll::SDL_SCANCODE_KP_SPACE           => KpSpace,
            ll::SDL_SCANCODE_KP_AT              => KpAt,
            ll::SDL_SCANCODE_KP_EXCLAM          => KpExclam,
            ll::SDL_SCANCODE_KP_MEMSTORE        => KpMemStore,
            ll::SDL_SCANCODE_KP_MEMRECALL       => KpMemRecall,
            ll::SDL_SCANCODE_KP_MEMCLEAR        => KpMemClear,
            ll::SDL_SCANCODE_KP_MEMADD          => KpMemAdd,
            ll::SDL_SCANCODE_KP_MEMSUBTRACT     => KpMemSubtract,
            ll::SDL_SCANCODE_KP_MEMMULTIPLY     => KpMemMultiply,
            ll::SDL_SCANCODE_KP_MEMDIVIDE       => KpMemDivide,
            ll::SDL_SCANCODE_KP_PLUSMINUS       => KpPlusMinus,
            ll::SDL_SCANCODE_KP_CLEAR           => KpClear,
            ll::SDL_SCANCODE_KP_CLEARENTRY      => KpClearEntry,
            ll::SDL_SCANCODE_KP_BINARY          => KpBinary,
            ll::SDL_SCANCODE_KP_OCTAL           => KpOctal,
            ll::SDL_SCANCODE_KP_DECIMAL         => KpDecimal,
            ll::SDL_SCANCODE_KP_HEXADECIMAL     => KpHexadecimal,
            ll::SDL_SCANCODE_LCTRL              => LCtrl,
            ll::SDL_SCANCODE_LSHIFT             => LShift,
            ll::SDL_SCANCODE_LALT               => LAlt,
            ll::SDL_SCANCODE_LGUI               => LGui,
            ll::SDL_SCANCODE_RCTRL              => RCtrl,
            ll::SDL_SCANCODE_RSHIFT             => RShift,
            ll::SDL_SCANCODE_RALT               => RAlt,
            ll::SDL_SCANCODE_RGUI               => RGui,
            ll::SDL_SCANCODE_MODE               => Mode,
            ll::SDL_SCANCODE_AUDIONEXT          => AudioNext,
            ll::SDL_SCANCODE_AUDIOPREV          => AudioPrev,
            ll::SDL_SCANCODE_AUDIOSTOP          => AudioStop,
            ll::SDL_SCANCODE_AUDIOPLAY          => AudioPlay,
            ll::SDL_SCANCODE_AUDIOMUTE          => AudioMute,
            ll::SDL_SCANCODE_MEDIASELECT        => MediaSelect,
            ll::SDL_SCANCODE_WWW                => Www,
            ll::SDL_SCANCODE_MAIL               => Mail,
            ll::SDL_SCANCODE_CALCULATOR         => Calculator,
            ll::SDL_SCANCODE_COMPUTER           => Computer,
            ll::SDL_SCANCODE_AC_SEARCH          => AcSearch,
            ll::SDL_SCANCODE_AC_HOME            => AcHome,
            ll::SDL_SCANCODE_AC_BACK            => AcBack,
            ll::SDL_SCANCODE_AC_FORWARD         => AcForward,
            ll::SDL_SCANCODE_AC_STOP            => AcStop,
            ll::SDL_SCANCODE_AC_REFRESH         => AcRefresh,
            ll::SDL_SCANCODE_AC_BOOKMARKS       => AcBookmarks,
            ll::SDL_SCANCODE_BRIGHTNESSDOWN     => BrightnessDown,
            ll::SDL_SCANCODE_BRIGHTNESSUP       => BrightnessUp,
            ll::SDL_SCANCODE_DISPLAYSWITCH      => DisplaySwitch,
            ll::SDL_SCANCODE_KBDILLUMTOGGLE     => KbdIllumToggle,
            ll::SDL_SCANCODE_KBDILLUMDOWN       => KbdIllumDown,
            ll::SDL_SCANCODE_KBDILLUMUP         => KbdIllumUp,
            ll::SDL_SCANCODE_EJECT              => Eject,
            ll::SDL_SCANCODE_SLEEP              => Sleep,
            ll::SDL_SCANCODE_APP1               => App1,
            ll::SDL_SCANCODE_APP2               => App2,
            ll::SDL_NUM_SCANCODES               => Num,
            _                                   => return None,
        })
    }
}

use std::fmt;

impl fmt::Display for Scancode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name())
    }
}

use keyboard::Keycode;

impl Scancode {
    /// Gets the scancode from a virtual key. Returns None if there is no corresponding scancode.
    pub fn from_keycode(keycode: Keycode) -> Option<Scancode> {
        unsafe {
            match ::sys::keyboard::SDL_GetScancodeFromKey(keycode as i32) {
                ll::SDL_SCANCODE_UNKNOWN => None,
                scancode_id => Scancode::from_i32(scancode_id as i32)
            }
        }
    }

    pub fn from_name(name: &str) -> Option<Scancode> {
        unsafe {
            match CString::new(name) {
                Ok(name) => match ::sys::keyboard::SDL_GetScancodeFromName(name.as_ptr() as *const c_char) {
                    ll::SDL_SCANCODE_UNKNOWN => None,
                    scancode_id => Some(Scancode::from_i32(scancode_id as i32).unwrap())
                },
                // string contains a nul byte - it won't match anything.
                Err(_) => None
            }
        }
    }

    pub fn name(self) -> &'static str {
        // The name string pointer lives in static, read-only memory.
        // Knowing this, we can always return a string slice.
        unsafe {
            let buf = ::sys::keyboard::SDL_GetScancodeName(self as u32);
            ::std::str::from_utf8(CStr::from_ptr(buf as *const _).to_bytes()).unwrap()
        }
    }
}
