use crate::public::{
    KeybdKey::{self, *},
    MouseButton::{self, *},
};

impl From<KeybdKey> for u64 {
    // https://www.win.tue.nl/~aeb/linux/kbd/scancodes-1.html
    fn from(key: KeybdKey) -> u64 {
        match key {
            BackspaceKey => 0xFF08,
            TabKey => 0xFF09,
            EnterKey => 0xFF8D,
            EscapeKey => 0xFF1B,
            SpaceKey => 0x020,
            HomeKey => 0xFF50,
            LeftKey => 0xFF51,
            UpKey => 0xFF52,
            RightKey => 0xFF53,
            DownKey => 0xFF54,
            InsertKey => 0xFF63,
            DeleteKey => 0xFF9F,
            Numrow0Key => 0x030,
            Numrow1Key => 0x031,
            Numrow2Key => 0x032,
            Numrow3Key => 0x033,
            Numrow4Key => 0x034,
            Numrow5Key => 0x035,
            Numrow6Key => 0x036,
            Numrow7Key => 0x037,
            Numrow8Key => 0x038,
            Numrow9Key => 0x039,
            AKey => 0x041,
            BKey => 0x042,
            CKey => 0x043,
            DKey => 0x044,
            EKey => 0x045,
            FKey => 0x046,
            GKey => 0x047,
            HKey => 0x048,
            IKey => 0x049,
            JKey => 0x04A,
            KKey => 0x04B,
            LKey => 0x04C,
            MKey => 0x04D,
            NKey => 0x04E,
            OKey => 0x04F,
            PKey => 0x050,
            QKey => 0x051,
            RKey => 0x052,
            SKey => 0x053,
            TKey => 0x054,
            UKey => 0x055,
            VKey => 0x056,
            WKey => 0x057,
            XKey => 0x058,
            YKey => 0x059,
            ZKey => 0x05A,
            Numpad0Key => 0xFFB0,
            Numpad1Key => 0xFFB1,
            Numpad2Key => 0xFFB2,
            Numpad3Key => 0xFFB3,
            Numpad4Key => 0xFFB4,
            Numpad5Key => 0xFFB5,
            Numpad6Key => 0xFFB6,
            Numpad7Key => 0xFFB7,
            Numpad8Key => 0xFFB8,
            Numpad9Key => 0xFFB9,
            F1Key => 0xFFBE,
            F2Key => 0xFFBF,
            F3Key => 0xFFC0,
            F4Key => 0xFFC1,
            F5Key => 0xFFC2,
            F6Key => 0xFFC3,
            F7Key => 0xFFC4,
            F8Key => 0xFFC5,
            F9Key => 0xFFC6,
            F10Key => 0xFFC7,
            F11Key => 0xFFC8,
            F12Key => 0xFFC9,
            F13Key => 0xFFCA,
            F14Key => 0xFFCB,
            F15Key => 0xFFCC,
            F16Key => 0xFFCD,
            F17Key => 0xFFCE,
            F18Key => 0xFFCF,
            F19Key => 0xFFD0,
            F20Key => 0xFFD1,
            F21Key => 0xFFD2,
            F22Key => 0xFFD3,
            F23Key => 0xFFD4,
            F24Key => 0xFFD5,
            NumLockKey => 0xFF7F,
            ScrollLockKey => 0xFF14,
            CapsLockKey => 0xFFE5,
            LShiftKey => 0xFFE1,
            RShiftKey => 0xFFE2,
            LControlKey => 0xFFE3,
            RControlKey => 0xFFE4,
            BackquoteKey => 0x29,
            SlashKey => 0x35,
            BackslashKey => 0x2B,
            CommaKey => 0x33,
            PeriodKey => 0x34,
            MinusKey => 0x0C,
            QuoteKey => 0x28,
            SemicolonKey => 0x27,
            LBracketKey => 0x1A,
            RBracketKey => 0x1B,
            EqualKey => 0x0D,
            OtherKey(keycode) => keycode,
        }
    }
}

impl From<u32> for MouseButton {
    fn from(keycode: u32) -> MouseButton {
        match keycode {
            1 => LeftButton,
            2 => MiddleButton,
            3 => RightButton,
            4 => X1Button,
            5 => X2Button,
            _ => OtherButton(keycode),
        }
    }
}

impl From<MouseButton> for u32 {
    fn from(button: MouseButton) -> u32 {
        match button {
            LeftButton => 1,
            MiddleButton => 2,
            RightButton => 3,
            X1Button => 4,
            X2Button => 5,
            OtherButton(keycode) => keycode,
        }
    }
}

// https://www.win.tue.nl/~aeb/linux/kbd/scancodes-1.html
pub fn scan_code_to_key(scan_code: u32) -> Option<KeybdKey> {
    match scan_code {
        0x0E => Some(BackspaceKey),
        0x0F => Some(TabKey),
        0x1C => Some(EnterKey),
        0x01 => Some(EscapeKey),
        0x39 => Some(SpaceKey),
        0x47 => Some(HomeKey),
        0x4B => Some(LeftKey),
        0x48 => Some(UpKey),
        0x4D => Some(RightKey),
        0x50 => Some(DownKey),
        0x52 => Some(InsertKey),
        0x53 => Some(DeleteKey),
        0x0B => Some(Numrow0Key),
        0x02 => Some(Numrow1Key),
        0x03 => Some(Numrow2Key),
        0x04 => Some(Numrow3Key),
        0x05 => Some(Numrow4Key),
        0x06 => Some(Numrow5Key),
        0x07 => Some(Numrow6Key),
        0x08 => Some(Numrow7Key),
        0x09 => Some(Numrow8Key),
        0x0A => Some(Numrow9Key),
        0x1E => Some(AKey),
        0x30 => Some(BKey),
        0x2E => Some(CKey),
        0x20 => Some(DKey),
        0x12 => Some(EKey),
        0x21 => Some(FKey),
        0x22 => Some(GKey),
        0x23 => Some(HKey),
        0x17 => Some(IKey),
        0x24 => Some(JKey),
        0x25 => Some(KKey),
        0x26 => Some(LKey),
        0x32 => Some(MKey),
        0x31 => Some(NKey),
        0x18 => Some(OKey),
        0x19 => Some(PKey),
        0x10 => Some(QKey),
        0x13 => Some(RKey),
        0x1F => Some(SKey),
        0x14 => Some(TKey),
        0x16 => Some(UKey),
        0x2F => Some(VKey),
        0x11 => Some(WKey),
        0x2D => Some(XKey),
        0x15 => Some(YKey),
        0x2C => Some(ZKey),
        0x52 => Some(Numpad0Key),
        0x4F => Some(Numpad1Key),
        0x50 => Some(Numpad2Key),
        0x51 => Some(Numpad3Key),
        0x4B => Some(Numpad4Key),
        0x4C => Some(Numpad5Key),
        0x4D => Some(Numpad6Key),
        0x47 => Some(Numpad7Key),
        0x48 => Some(Numpad8Key),
        0x49 => Some(Numpad9Key),
        0x3B => Some(F1Key),
        0x3C => Some(F2Key),
        0x3D => Some(F3Key),
        0x3E => Some(F4Key),
        0x3F => Some(F5Key),
        0x40 => Some(F6Key),
        0x41 => Some(F7Key),
        0x42 => Some(F8Key),
        0x43 => Some(F9Key),
        0x44 => Some(F10Key),
        0x45 => Some(NumLockKey),
        0x46 => Some(ScrollLockKey),
        0x3A => Some(CapsLockKey),
        0x2A => Some(LShiftKey),
        0x36 => Some(RShiftKey),
        0x1D => Some(LControlKey),
        0x29 => Some(BackquoteKey),
        0x35 => Some(SlashKey),
        0x2B => Some(BackslashKey),
        0x33 => Some(CommaKey),
        0x34 => Some(PeriodKey),
        0x0C => Some(MinusKey),
        0x28 => Some(QuoteKey),
        0x27 => Some(SemicolonKey),
        0x1A => Some(LBracketKey),
        0x1B => Some(RBracketKey),
        0x0D => Some(EqualKey),
        _ => None,
    }
}

// https://www.win.tue.nl/~aeb/linux/kbd/scancodes-1.html
pub fn key_to_scan_code(key: KeybdKey) -> i32 {
    match key {
        BackspaceKey => 0x0E,
        TabKey => 0x0F,
        EnterKey => 0x1C,
        EscapeKey => 0x01,
        SpaceKey => 0x39,
        HomeKey => 0x47,
        LeftKey => 0x4b,
        UpKey => 0x48,
        RightKey => 0x4D,
        DownKey => 0x50,
        InsertKey => 0x52,
        DeleteKey => 0x53,
        Numrow0Key => 0x0B,
        Numrow1Key => 0x02,
        Numrow2Key => 0x03,
        Numrow3Key => 0x04,
        Numrow4Key => 0x05,
        Numrow5Key => 0x06,
        Numrow6Key => 0x07,
        Numrow7Key => 0x08,
        Numrow8Key => 0x09,
        Numrow9Key => 0x0A,
        AKey => 0x1E,
        BKey => 0x30,
        CKey => 0x2E,
        DKey => 0x20,
        EKey => 0x12,
        FKey => 0x21,
        GKey => 0x22,
        HKey => 0x23,
        IKey => 0x17,
        JKey => 0x24,
        KKey => 0x25,
        LKey => 0x26,
        MKey => 0x32,
        NKey => 0x31,
        OKey => 0x18,
        PKey => 0x19,
        QKey => 0x10,
        RKey => 0x13,
        SKey => 0x1F,
        TKey => 0x14,
        UKey => 0x16,
        VKey => 0x2F,
        WKey => 0x11,
        XKey => 0x2D,
        YKey => 0x15,
        ZKey => 0x2C,
        Numpad0Key => 0x52,
        Numpad1Key => 0x4F,
        Numpad2Key => 0x50,
        Numpad3Key => 0x51,
        Numpad4Key => 0x4B,
        Numpad5Key => 0x4C,
        Numpad6Key => 0x4D,
        Numpad7Key => 0x47,
        Numpad8Key => 0x48,
        Numpad9Key => 0x49,
        F1Key => 0x3B,
        F2Key => 0x3C,
        F3Key => 0x3D,
        F4Key => 0x3E,
        F5Key => 0x3F,
        F6Key => 0x40,
        F7Key => 0x41,
        F8Key => 0x42,
        F9Key => 0x43,
        F10Key => 0x44,
        NumLockKey => 0x45,
        ScrollLockKey => 0x46,
        CapsLockKey => 0x3A,
        LShiftKey => 0x2A,
        RShiftKey => 0x36,
        LControlKey => 0x1D,
        BackquoteKey => 0x29,
        SlashKey => 0x35,
        BackslashKey => 0x2B,
        CommaKey => 0x33,
        PeriodKey => 0x34,
        MinusKey => 0x0C,
        QuoteKey => 0x28,
        SemicolonKey => 0x27,
        LBracketKey => 0x1A,
        RBracketKey => 0x1B,
        EqualKey => 0x0D,
        OtherKey(code) => code as i32,
        _ => 0x0,
    }
}

impl From<MouseButton> for uinput::event::controller::Mouse {
    fn from(button: MouseButton) -> Self {
        use uinput::event::controller::Mouse;
        match button {
            MouseButton::LeftButton => Mouse::Left,
            MouseButton::RightButton => Mouse::Right,
            MouseButton::MiddleButton => Mouse::Middle,
            MouseButton::X1Button => unimplemented!(),
            MouseButton::X2Button => unimplemented!(),
            MouseButton::OtherButton(_) => unimplemented!(),
        }
    }
}
