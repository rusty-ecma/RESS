#![allow(clippy::all)]

#[inline]
pub(crate) fn is_id_start(c: char) -> bool {
    if c < '\u{41}' {
        return false;
    } else if c >= '\u{41}' && c <= '\u{5A}' {
        return true;
    } else if c < '\u{61}' {
        return false;
    } else if c >= '\u{61}' && c <= '\u{7A}' {
        return true;
    } else if c < '\u{AA}' {
        return false;
    } else if c >= '\u{AA}' && c <= '\u{AA}' {
        return true;
    } else if c < '\u{B5}' {
        return false;
    } else if c >= '\u{B5}' && c <= '\u{B5}' {
        return true;
    } else if c < '\u{BA}' {
        return false;
    } else if c >= '\u{BA}' && c <= '\u{BA}' {
        return true;
    } else if c < '\u{C0}' {
        return false;
    } else if c >= '\u{C0}' && c <= '\u{D6}' {
        return true;
    } else if c < '\u{D8}' {
        return false;
    } else if c >= '\u{D8}' && c <= '\u{F6}' {
        return true;
    } else if c < '\u{F8}' {
        return false;
    } else if c >= '\u{F8}' && c <= '\u{2C1}' {
        return true;
    } else if c < '\u{2C6}' {
        return false;
    } else if c >= '\u{2C6}' && c <= '\u{2D1}' {
        return true;
    } else if c < '\u{2E0}' {
        return false;
    } else if c >= '\u{2E0}' && c <= '\u{2E4}' {
        return true;
    } else if c < '\u{2EC}' {
        return false;
    } else if c >= '\u{2EC}' && c <= '\u{2EC}' {
        return true;
    } else if c < '\u{2EE}' {
        return false;
    } else if c >= '\u{2EE}' && c <= '\u{2EE}' {
        return true;
    } else if c < '\u{370}' {
        return false;
    } else if c >= '\u{370}' && c <= '\u{374}' {
        return true;
    } else if c < '\u{376}' {
        return false;
    } else if c >= '\u{376}' && c <= '\u{377}' {
        return true;
    } else if c < '\u{37A}' {
        return false;
    } else if c >= '\u{37A}' && c <= '\u{37D}' {
        return true;
    } else if c < '\u{37F}' {
        return false;
    } else if c >= '\u{37F}' && c <= '\u{37F}' {
        return true;
    } else if c < '\u{386}' {
        return false;
    } else if c >= '\u{386}' && c <= '\u{386}' {
        return true;
    } else if c < '\u{388}' {
        return false;
    } else if c >= '\u{388}' && c <= '\u{38A}' {
        return true;
    } else if c < '\u{38C}' {
        return false;
    } else if c >= '\u{38C}' && c <= '\u{38C}' {
        return true;
    } else if c < '\u{38E}' {
        return false;
    } else if c >= '\u{38E}' && c <= '\u{3A1}' {
        return true;
    } else if c < '\u{3A3}' {
        return false;
    } else if c >= '\u{3A3}' && c <= '\u{3F5}' {
        return true;
    } else if c < '\u{3F7}' {
        return false;
    } else if c >= '\u{3F7}' && c <= '\u{481}' {
        return true;
    } else if c < '\u{48A}' {
        return false;
    } else if c >= '\u{48A}' && c <= '\u{52F}' {
        return true;
    } else if c < '\u{531}' {
        return false;
    } else if c >= '\u{531}' && c <= '\u{556}' {
        return true;
    } else if c < '\u{559}' {
        return false;
    } else if c >= '\u{559}' && c <= '\u{559}' {
        return true;
    } else if c < '\u{561}' {
        return false;
    } else if c >= '\u{561}' && c <= '\u{587}' {
        return true;
    } else if c < '\u{5D0}' {
        return false;
    } else if c >= '\u{5D0}' && c <= '\u{5EA}' {
        return true;
    } else if c < '\u{5F0}' {
        return false;
    } else if c >= '\u{5F0}' && c <= '\u{5F2}' {
        return true;
    } else if c < '\u{620}' {
        return false;
    } else if c >= '\u{620}' && c <= '\u{64A}' {
        return true;
    } else if c < '\u{66E}' {
        return false;
    } else if c >= '\u{66E}' && c <= '\u{66F}' {
        return true;
    } else if c < '\u{671}' {
        return false;
    } else if c >= '\u{671}' && c <= '\u{6D3}' {
        return true;
    } else if c < '\u{6D5}' {
        return false;
    } else if c >= '\u{6D5}' && c <= '\u{6D5}' {
        return true;
    } else if c < '\u{6E5}' {
        return false;
    } else if c >= '\u{6E5}' && c <= '\u{6E6}' {
        return true;
    } else if c < '\u{6EE}' {
        return false;
    } else if c >= '\u{6EE}' && c <= '\u{6EF}' {
        return true;
    } else if c < '\u{6FA}' {
        return false;
    } else if c >= '\u{6FA}' && c <= '\u{6FC}' {
        return true;
    } else if c < '\u{6FF}' {
        return false;
    } else if c >= '\u{6FF}' && c <= '\u{6FF}' {
        return true;
    } else if c < '\u{710}' {
        return false;
    } else if c >= '\u{710}' && c <= '\u{710}' {
        return true;
    } else if c < '\u{712}' {
        return false;
    } else if c >= '\u{712}' && c <= '\u{72F}' {
        return true;
    } else if c < '\u{74D}' {
        return false;
    } else if c >= '\u{74D}' && c <= '\u{7A5}' {
        return true;
    } else if c < '\u{7B1}' {
        return false;
    } else if c >= '\u{7B1}' && c <= '\u{7B1}' {
        return true;
    } else if c < '\u{7CA}' {
        return false;
    } else if c >= '\u{7CA}' && c <= '\u{7EA}' {
        return true;
    } else if c < '\u{7F4}' {
        return false;
    } else if c >= '\u{7F4}' && c <= '\u{7F5}' {
        return true;
    } else if c < '\u{7FA}' {
        return false;
    } else if c >= '\u{7FA}' && c <= '\u{7FA}' {
        return true;
    } else if c < '\u{800}' {
        return false;
    } else if c >= '\u{800}' && c <= '\u{815}' {
        return true;
    } else if c < '\u{81A}' {
        return false;
    } else if c >= '\u{81A}' && c <= '\u{81A}' {
        return true;
    } else if c < '\u{824}' {
        return false;
    } else if c >= '\u{824}' && c <= '\u{824}' {
        return true;
    } else if c < '\u{828}' {
        return false;
    } else if c >= '\u{828}' && c <= '\u{828}' {
        return true;
    } else if c < '\u{840}' {
        return false;
    } else if c >= '\u{840}' && c <= '\u{858}' {
        return true;
    } else if c < '\u{8A0}' {
        return false;
    } else if c >= '\u{8A0}' && c <= '\u{8B4}' {
        return true;
    } else if c < '\u{8B6}' {
        return false;
    } else if c >= '\u{8B6}' && c <= '\u{8BD}' {
        return true;
    } else if c < '\u{904}' {
        return false;
    } else if c >= '\u{904}' && c <= '\u{939}' {
        return true;
    } else if c < '\u{93D}' {
        return false;
    } else if c >= '\u{93D}' && c <= '\u{93D}' {
        return true;
    } else if c < '\u{950}' {
        return false;
    } else if c >= '\u{950}' && c <= '\u{950}' {
        return true;
    } else if c < '\u{958}' {
        return false;
    } else if c >= '\u{958}' && c <= '\u{961}' {
        return true;
    } else if c < '\u{971}' {
        return false;
    } else if c >= '\u{971}' && c <= '\u{980}' {
        return true;
    } else if c < '\u{985}' {
        return false;
    } else if c >= '\u{985}' && c <= '\u{98C}' {
        return true;
    } else if c < '\u{98F}' {
        return false;
    } else if c >= '\u{98F}' && c <= '\u{990}' {
        return true;
    } else if c < '\u{993}' {
        return false;
    } else if c >= '\u{993}' && c <= '\u{9A8}' {
        return true;
    } else if c < '\u{9AA}' {
        return false;
    } else if c >= '\u{9AA}' && c <= '\u{9B0}' {
        return true;
    } else if c < '\u{9B2}' {
        return false;
    } else if c >= '\u{9B2}' && c <= '\u{9B2}' {
        return true;
    } else if c < '\u{9B6}' {
        return false;
    } else if c >= '\u{9B6}' && c <= '\u{9B9}' {
        return true;
    } else if c < '\u{9BD}' {
        return false;
    } else if c >= '\u{9BD}' && c <= '\u{9BD}' {
        return true;
    } else if c < '\u{9CE}' {
        return false;
    } else if c >= '\u{9CE}' && c <= '\u{9CE}' {
        return true;
    } else if c < '\u{9DC}' {
        return false;
    } else if c >= '\u{9DC}' && c <= '\u{9DD}' {
        return true;
    } else if c < '\u{9DF}' {
        return false;
    } else if c >= '\u{9DF}' && c <= '\u{9E1}' {
        return true;
    } else if c < '\u{9F0}' {
        return false;
    } else if c >= '\u{9F0}' && c <= '\u{9F1}' {
        return true;
    } else if c < '\u{A05}' {
        return false;
    } else if c >= '\u{A05}' && c <= '\u{A0A}' {
        return true;
    } else if c < '\u{A0F}' {
        return false;
    } else if c >= '\u{A0F}' && c <= '\u{A10}' {
        return true;
    } else if c < '\u{A13}' {
        return false;
    } else if c >= '\u{A13}' && c <= '\u{A28}' {
        return true;
    } else if c < '\u{A2A}' {
        return false;
    } else if c >= '\u{A2A}' && c <= '\u{A30}' {
        return true;
    } else if c < '\u{A32}' {
        return false;
    } else if c >= '\u{A32}' && c <= '\u{A33}' {
        return true;
    } else if c < '\u{A35}' {
        return false;
    } else if c >= '\u{A35}' && c <= '\u{A36}' {
        return true;
    } else if c < '\u{A38}' {
        return false;
    } else if c >= '\u{A38}' && c <= '\u{A39}' {
        return true;
    } else if c < '\u{A59}' {
        return false;
    } else if c >= '\u{A59}' && c <= '\u{A5C}' {
        return true;
    } else if c < '\u{A5E}' {
        return false;
    } else if c >= '\u{A5E}' && c <= '\u{A5E}' {
        return true;
    } else if c < '\u{A72}' {
        return false;
    } else if c >= '\u{A72}' && c <= '\u{A74}' {
        return true;
    } else if c < '\u{A85}' {
        return false;
    } else if c >= '\u{A85}' && c <= '\u{A8D}' {
        return true;
    } else if c < '\u{A8F}' {
        return false;
    } else if c >= '\u{A8F}' && c <= '\u{A91}' {
        return true;
    } else if c < '\u{A93}' {
        return false;
    } else if c >= '\u{A93}' && c <= '\u{AA8}' {
        return true;
    } else if c < '\u{AAA}' {
        return false;
    } else if c >= '\u{AAA}' && c <= '\u{AB0}' {
        return true;
    } else if c < '\u{AB2}' {
        return false;
    } else if c >= '\u{AB2}' && c <= '\u{AB3}' {
        return true;
    } else if c < '\u{AB5}' {
        return false;
    } else if c >= '\u{AB5}' && c <= '\u{AB9}' {
        return true;
    } else if c < '\u{ABD}' {
        return false;
    } else if c >= '\u{ABD}' && c <= '\u{ABD}' {
        return true;
    } else if c < '\u{AD0}' {
        return false;
    } else if c >= '\u{AD0}' && c <= '\u{AD0}' {
        return true;
    } else if c < '\u{AE0}' {
        return false;
    } else if c >= '\u{AE0}' && c <= '\u{AE1}' {
        return true;
    } else if c < '\u{AF9}' {
        return false;
    } else if c >= '\u{AF9}' && c <= '\u{AF9}' {
        return true;
    } else if c < '\u{B05}' {
        return false;
    } else if c >= '\u{B05}' && c <= '\u{B0C}' {
        return true;
    } else if c < '\u{B0F}' {
        return false;
    } else if c >= '\u{B0F}' && c <= '\u{B10}' {
        return true;
    } else if c < '\u{B13}' {
        return false;
    } else if c >= '\u{B13}' && c <= '\u{B28}' {
        return true;
    } else if c < '\u{B2A}' {
        return false;
    } else if c >= '\u{B2A}' && c <= '\u{B30}' {
        return true;
    } else if c < '\u{B32}' {
        return false;
    } else if c >= '\u{B32}' && c <= '\u{B33}' {
        return true;
    } else if c < '\u{B35}' {
        return false;
    } else if c >= '\u{B35}' && c <= '\u{B39}' {
        return true;
    } else if c < '\u{B3D}' {
        return false;
    } else if c >= '\u{B3D}' && c <= '\u{B3D}' {
        return true;
    } else if c < '\u{B5C}' {
        return false;
    } else if c >= '\u{B5C}' && c <= '\u{B5D}' {
        return true;
    } else if c < '\u{B5F}' {
        return false;
    } else if c >= '\u{B5F}' && c <= '\u{B61}' {
        return true;
    } else if c < '\u{B71}' {
        return false;
    } else if c >= '\u{B71}' && c <= '\u{B71}' {
        return true;
    } else if c < '\u{B83}' {
        return false;
    } else if c >= '\u{B83}' && c <= '\u{B83}' {
        return true;
    } else if c < '\u{B85}' {
        return false;
    } else if c >= '\u{B85}' && c <= '\u{B8A}' {
        return true;
    } else if c < '\u{B8E}' {
        return false;
    } else if c >= '\u{B8E}' && c <= '\u{B90}' {
        return true;
    } else if c < '\u{B92}' {
        return false;
    } else if c >= '\u{B92}' && c <= '\u{B95}' {
        return true;
    } else if c < '\u{B99}' {
        return false;
    } else if c >= '\u{B99}' && c <= '\u{B9A}' {
        return true;
    } else if c < '\u{B9C}' {
        return false;
    } else if c >= '\u{B9C}' && c <= '\u{B9C}' {
        return true;
    } else if c < '\u{B9E}' {
        return false;
    } else if c >= '\u{B9E}' && c <= '\u{B9F}' {
        return true;
    } else if c < '\u{BA3}' {
        return false;
    } else if c >= '\u{BA3}' && c <= '\u{BA4}' {
        return true;
    } else if c < '\u{BA8}' {
        return false;
    } else if c >= '\u{BA8}' && c <= '\u{BAA}' {
        return true;
    } else if c < '\u{BAE}' {
        return false;
    } else if c >= '\u{BAE}' && c <= '\u{BB9}' {
        return true;
    } else if c < '\u{BD0}' {
        return false;
    } else if c >= '\u{BD0}' && c <= '\u{BD0}' {
        return true;
    } else if c < '\u{C05}' {
        return false;
    } else if c >= '\u{C05}' && c <= '\u{C0C}' {
        return true;
    } else if c < '\u{C0E}' {
        return false;
    } else if c >= '\u{C0E}' && c <= '\u{C10}' {
        return true;
    } else if c < '\u{C12}' {
        return false;
    } else if c >= '\u{C12}' && c <= '\u{C28}' {
        return true;
    } else if c < '\u{C2A}' {
        return false;
    } else if c >= '\u{C2A}' && c <= '\u{C39}' {
        return true;
    } else if c < '\u{C3D}' {
        return false;
    } else if c >= '\u{C3D}' && c <= '\u{C3D}' {
        return true;
    } else if c < '\u{C58}' {
        return false;
    } else if c >= '\u{C58}' && c <= '\u{C5A}' {
        return true;
    } else if c < '\u{C60}' {
        return false;
    } else if c >= '\u{C60}' && c <= '\u{C61}' {
        return true;
    } else if c < '\u{C80}' {
        return false;
    } else if c >= '\u{C80}' && c <= '\u{C80}' {
        return true;
    } else if c < '\u{C85}' {
        return false;
    } else if c >= '\u{C85}' && c <= '\u{C8C}' {
        return true;
    } else if c < '\u{C8E}' {
        return false;
    } else if c >= '\u{C8E}' && c <= '\u{C90}' {
        return true;
    } else if c < '\u{C92}' {
        return false;
    } else if c >= '\u{C92}' && c <= '\u{CA8}' {
        return true;
    } else if c < '\u{CAA}' {
        return false;
    } else if c >= '\u{CAA}' && c <= '\u{CB3}' {
        return true;
    } else if c < '\u{CB5}' {
        return false;
    } else if c >= '\u{CB5}' && c <= '\u{CB9}' {
        return true;
    } else if c < '\u{CBD}' {
        return false;
    } else if c >= '\u{CBD}' && c <= '\u{CBD}' {
        return true;
    } else if c < '\u{CDE}' {
        return false;
    } else if c >= '\u{CDE}' && c <= '\u{CDE}' {
        return true;
    } else if c < '\u{CE0}' {
        return false;
    } else if c >= '\u{CE0}' && c <= '\u{CE1}' {
        return true;
    } else if c < '\u{CF1}' {
        return false;
    } else if c >= '\u{CF1}' && c <= '\u{CF2}' {
        return true;
    } else if c < '\u{D05}' {
        return false;
    } else if c >= '\u{D05}' && c <= '\u{D0C}' {
        return true;
    } else if c < '\u{D0E}' {
        return false;
    } else if c >= '\u{D0E}' && c <= '\u{D10}' {
        return true;
    } else if c < '\u{D12}' {
        return false;
    } else if c >= '\u{D12}' && c <= '\u{D3A}' {
        return true;
    } else if c < '\u{D3D}' {
        return false;
    } else if c >= '\u{D3D}' && c <= '\u{D3D}' {
        return true;
    } else if c < '\u{D4E}' {
        return false;
    } else if c >= '\u{D4E}' && c <= '\u{D4E}' {
        return true;
    } else if c < '\u{D54}' {
        return false;
    } else if c >= '\u{D54}' && c <= '\u{D56}' {
        return true;
    } else if c < '\u{D5F}' {
        return false;
    } else if c >= '\u{D5F}' && c <= '\u{D61}' {
        return true;
    } else if c < '\u{D7A}' {
        return false;
    } else if c >= '\u{D7A}' && c <= '\u{D7F}' {
        return true;
    } else if c < '\u{D85}' {
        return false;
    } else if c >= '\u{D85}' && c <= '\u{D96}' {
        return true;
    } else if c < '\u{D9A}' {
        return false;
    } else if c >= '\u{D9A}' && c <= '\u{DB1}' {
        return true;
    } else if c < '\u{DB3}' {
        return false;
    } else if c >= '\u{DB3}' && c <= '\u{DBB}' {
        return true;
    } else if c < '\u{DBD}' {
        return false;
    } else if c >= '\u{DBD}' && c <= '\u{DBD}' {
        return true;
    } else if c < '\u{DC0}' {
        return false;
    } else if c >= '\u{DC0}' && c <= '\u{DC6}' {
        return true;
    } else if c < '\u{E01}' {
        return false;
    } else if c >= '\u{E01}' && c <= '\u{E30}' {
        return true;
    } else if c < '\u{E32}' {
        return false;
    } else if c >= '\u{E32}' && c <= '\u{E33}' {
        return true;
    } else if c < '\u{E40}' {
        return false;
    } else if c >= '\u{E40}' && c <= '\u{E46}' {
        return true;
    } else if c < '\u{E81}' {
        return false;
    } else if c >= '\u{E81}' && c <= '\u{E82}' {
        return true;
    } else if c < '\u{E84}' {
        return false;
    } else if c >= '\u{E84}' && c <= '\u{E84}' {
        return true;
    } else if c < '\u{E87}' {
        return false;
    } else if c >= '\u{E87}' && c <= '\u{E88}' {
        return true;
    } else if c < '\u{E8A}' {
        return false;
    } else if c >= '\u{E8A}' && c <= '\u{E8A}' {
        return true;
    } else if c < '\u{E8D}' {
        return false;
    } else if c >= '\u{E8D}' && c <= '\u{E8D}' {
        return true;
    } else if c < '\u{E94}' {
        return false;
    } else if c >= '\u{E94}' && c <= '\u{E97}' {
        return true;
    } else if c < '\u{E99}' {
        return false;
    } else if c >= '\u{E99}' && c <= '\u{E9F}' {
        return true;
    } else if c < '\u{EA1}' {
        return false;
    } else if c >= '\u{EA1}' && c <= '\u{EA3}' {
        return true;
    } else if c < '\u{EA5}' {
        return false;
    } else if c >= '\u{EA5}' && c <= '\u{EA5}' {
        return true;
    } else if c < '\u{EA7}' {
        return false;
    } else if c >= '\u{EA7}' && c <= '\u{EA7}' {
        return true;
    } else if c < '\u{EAA}' {
        return false;
    } else if c >= '\u{EAA}' && c <= '\u{EAB}' {
        return true;
    } else if c < '\u{EAD}' {
        return false;
    } else if c >= '\u{EAD}' && c <= '\u{EB0}' {
        return true;
    } else if c < '\u{EB2}' {
        return false;
    } else if c >= '\u{EB2}' && c <= '\u{EB3}' {
        return true;
    } else if c < '\u{EBD}' {
        return false;
    } else if c >= '\u{EBD}' && c <= '\u{EBD}' {
        return true;
    } else if c < '\u{EC0}' {
        return false;
    } else if c >= '\u{EC0}' && c <= '\u{EC4}' {
        return true;
    } else if c < '\u{EC6}' {
        return false;
    } else if c >= '\u{EC6}' && c <= '\u{EC6}' {
        return true;
    } else if c < '\u{EDC}' {
        return false;
    } else if c >= '\u{EDC}' && c <= '\u{EDF}' {
        return true;
    } else if c < '\u{F00}' {
        return false;
    } else if c >= '\u{F00}' && c <= '\u{F00}' {
        return true;
    } else if c < '\u{F40}' {
        return false;
    } else if c >= '\u{F40}' && c <= '\u{F47}' {
        return true;
    } else if c < '\u{F49}' {
        return false;
    } else if c >= '\u{F49}' && c <= '\u{F6C}' {
        return true;
    } else if c < '\u{F88}' {
        return false;
    } else if c >= '\u{F88}' && c <= '\u{F8C}' {
        return true;
    } else if c < '\u{1000}' {
        return false;
    } else if c >= '\u{1000}' && c <= '\u{102A}' {
        return true;
    } else if c < '\u{103F}' {
        return false;
    } else if c >= '\u{103F}' && c <= '\u{103F}' {
        return true;
    } else if c < '\u{1050}' {
        return false;
    } else if c >= '\u{1050}' && c <= '\u{1055}' {
        return true;
    } else if c < '\u{105A}' {
        return false;
    } else if c >= '\u{105A}' && c <= '\u{105D}' {
        return true;
    } else if c < '\u{1061}' {
        return false;
    } else if c >= '\u{1061}' && c <= '\u{1061}' {
        return true;
    } else if c < '\u{1065}' {
        return false;
    } else if c >= '\u{1065}' && c <= '\u{1066}' {
        return true;
    } else if c < '\u{106E}' {
        return false;
    } else if c >= '\u{106E}' && c <= '\u{1070}' {
        return true;
    } else if c < '\u{1075}' {
        return false;
    } else if c >= '\u{1075}' && c <= '\u{1081}' {
        return true;
    } else if c < '\u{108E}' {
        return false;
    } else if c >= '\u{108E}' && c <= '\u{108E}' {
        return true;
    } else if c < '\u{10A0}' {
        return false;
    } else if c >= '\u{10A0}' && c <= '\u{10C5}' {
        return true;
    } else if c < '\u{10C7}' {
        return false;
    } else if c >= '\u{10C7}' && c <= '\u{10C7}' {
        return true;
    } else if c < '\u{10CD}' {
        return false;
    } else if c >= '\u{10CD}' && c <= '\u{10CD}' {
        return true;
    } else if c < '\u{10D0}' {
        return false;
    } else if c >= '\u{10D0}' && c <= '\u{10FA}' {
        return true;
    } else if c < '\u{10FC}' {
        return false;
    } else if c >= '\u{10FC}' && c <= '\u{1248}' {
        return true;
    } else if c < '\u{124A}' {
        return false;
    } else if c >= '\u{124A}' && c <= '\u{124D}' {
        return true;
    } else if c < '\u{1250}' {
        return false;
    } else if c >= '\u{1250}' && c <= '\u{1256}' {
        return true;
    } else if c < '\u{1258}' {
        return false;
    } else if c >= '\u{1258}' && c <= '\u{1258}' {
        return true;
    } else if c < '\u{125A}' {
        return false;
    } else if c >= '\u{125A}' && c <= '\u{125D}' {
        return true;
    } else if c < '\u{1260}' {
        return false;
    } else if c >= '\u{1260}' && c <= '\u{1288}' {
        return true;
    } else if c < '\u{128A}' {
        return false;
    } else if c >= '\u{128A}' && c <= '\u{128D}' {
        return true;
    } else if c < '\u{1290}' {
        return false;
    } else if c >= '\u{1290}' && c <= '\u{12B0}' {
        return true;
    } else if c < '\u{12B2}' {
        return false;
    } else if c >= '\u{12B2}' && c <= '\u{12B5}' {
        return true;
    } else if c < '\u{12B8}' {
        return false;
    } else if c >= '\u{12B8}' && c <= '\u{12BE}' {
        return true;
    } else if c < '\u{12C0}' {
        return false;
    } else if c >= '\u{12C0}' && c <= '\u{12C0}' {
        return true;
    } else if c < '\u{12C2}' {
        return false;
    } else if c >= '\u{12C2}' && c <= '\u{12C5}' {
        return true;
    } else if c < '\u{12C8}' {
        return false;
    } else if c >= '\u{12C8}' && c <= '\u{12D6}' {
        return true;
    } else if c < '\u{12D8}' {
        return false;
    } else if c >= '\u{12D8}' && c <= '\u{1310}' {
        return true;
    } else if c < '\u{1312}' {
        return false;
    } else if c >= '\u{1312}' && c <= '\u{1315}' {
        return true;
    } else if c < '\u{1318}' {
        return false;
    } else if c >= '\u{1318}' && c <= '\u{135A}' {
        return true;
    } else if c < '\u{1380}' {
        return false;
    } else if c >= '\u{1380}' && c <= '\u{138F}' {
        return true;
    } else if c < '\u{13A0}' {
        return false;
    } else if c >= '\u{13A0}' && c <= '\u{13F5}' {
        return true;
    } else if c < '\u{13F8}' {
        return false;
    } else if c >= '\u{13F8}' && c <= '\u{13FD}' {
        return true;
    } else if c < '\u{1401}' {
        return false;
    } else if c >= '\u{1401}' && c <= '\u{166C}' {
        return true;
    } else if c < '\u{166F}' {
        return false;
    } else if c >= '\u{166F}' && c <= '\u{167F}' {
        return true;
    } else if c < '\u{1681}' {
        return false;
    } else if c >= '\u{1681}' && c <= '\u{169A}' {
        return true;
    } else if c < '\u{16A0}' {
        return false;
    } else if c >= '\u{16A0}' && c <= '\u{16EA}' {
        return true;
    } else if c < '\u{16EE}' {
        return false;
    } else if c >= '\u{16EE}' && c <= '\u{16F8}' {
        return true;
    } else if c < '\u{1700}' {
        return false;
    } else if c >= '\u{1700}' && c <= '\u{170C}' {
        return true;
    } else if c < '\u{170E}' {
        return false;
    } else if c >= '\u{170E}' && c <= '\u{1711}' {
        return true;
    } else if c < '\u{1720}' {
        return false;
    } else if c >= '\u{1720}' && c <= '\u{1731}' {
        return true;
    } else if c < '\u{1740}' {
        return false;
    } else if c >= '\u{1740}' && c <= '\u{1751}' {
        return true;
    } else if c < '\u{1760}' {
        return false;
    } else if c >= '\u{1760}' && c <= '\u{176C}' {
        return true;
    } else if c < '\u{176E}' {
        return false;
    } else if c >= '\u{176E}' && c <= '\u{1770}' {
        return true;
    } else if c < '\u{1780}' {
        return false;
    } else if c >= '\u{1780}' && c <= '\u{17B3}' {
        return true;
    } else if c < '\u{17D7}' {
        return false;
    } else if c >= '\u{17D7}' && c <= '\u{17D7}' {
        return true;
    } else if c < '\u{17DC}' {
        return false;
    } else if c >= '\u{17DC}' && c <= '\u{17DC}' {
        return true;
    } else if c < '\u{1820}' {
        return false;
    } else if c >= '\u{1820}' && c <= '\u{1877}' {
        return true;
    } else if c < '\u{1880}' {
        return false;
    } else if c >= '\u{1880}' && c <= '\u{18A8}' {
        return true;
    } else if c < '\u{18AA}' {
        return false;
    } else if c >= '\u{18AA}' && c <= '\u{18AA}' {
        return true;
    } else if c < '\u{18B0}' {
        return false;
    } else if c >= '\u{18B0}' && c <= '\u{18F5}' {
        return true;
    } else if c < '\u{1900}' {
        return false;
    } else if c >= '\u{1900}' && c <= '\u{191E}' {
        return true;
    } else if c < '\u{1950}' {
        return false;
    } else if c >= '\u{1950}' && c <= '\u{196D}' {
        return true;
    } else if c < '\u{1970}' {
        return false;
    } else if c >= '\u{1970}' && c <= '\u{1974}' {
        return true;
    } else if c < '\u{1980}' {
        return false;
    } else if c >= '\u{1980}' && c <= '\u{19AB}' {
        return true;
    } else if c < '\u{19B0}' {
        return false;
    } else if c >= '\u{19B0}' && c <= '\u{19C9}' {
        return true;
    } else if c < '\u{1A00}' {
        return false;
    } else if c >= '\u{1A00}' && c <= '\u{1A16}' {
        return true;
    } else if c < '\u{1A20}' {
        return false;
    } else if c >= '\u{1A20}' && c <= '\u{1A54}' {
        return true;
    } else if c < '\u{1AA7}' {
        return false;
    } else if c >= '\u{1AA7}' && c <= '\u{1AA7}' {
        return true;
    } else if c < '\u{1B05}' {
        return false;
    } else if c >= '\u{1B05}' && c <= '\u{1B33}' {
        return true;
    } else if c < '\u{1B45}' {
        return false;
    } else if c >= '\u{1B45}' && c <= '\u{1B4B}' {
        return true;
    } else if c < '\u{1B83}' {
        return false;
    } else if c >= '\u{1B83}' && c <= '\u{1BA0}' {
        return true;
    } else if c < '\u{1BAE}' {
        return false;
    } else if c >= '\u{1BAE}' && c <= '\u{1BAF}' {
        return true;
    } else if c < '\u{1BBA}' {
        return false;
    } else if c >= '\u{1BBA}' && c <= '\u{1BE5}' {
        return true;
    } else if c < '\u{1C00}' {
        return false;
    } else if c >= '\u{1C00}' && c <= '\u{1C23}' {
        return true;
    } else if c < '\u{1C4D}' {
        return false;
    } else if c >= '\u{1C4D}' && c <= '\u{1C4F}' {
        return true;
    } else if c < '\u{1C5A}' {
        return false;
    } else if c >= '\u{1C5A}' && c <= '\u{1C7D}' {
        return true;
    } else if c < '\u{1C80}' {
        return false;
    } else if c >= '\u{1C80}' && c <= '\u{1C88}' {
        return true;
    } else if c < '\u{1CE9}' {
        return false;
    } else if c >= '\u{1CE9}' && c <= '\u{1CEC}' {
        return true;
    } else if c < '\u{1CEE}' {
        return false;
    } else if c >= '\u{1CEE}' && c <= '\u{1CF1}' {
        return true;
    } else if c < '\u{1CF5}' {
        return false;
    } else if c >= '\u{1CF5}' && c <= '\u{1CF6}' {
        return true;
    } else if c < '\u{1D00}' {
        return false;
    } else if c >= '\u{1D00}' && c <= '\u{1DBF}' {
        return true;
    } else if c < '\u{1E00}' {
        return false;
    } else if c >= '\u{1E00}' && c <= '\u{1F15}' {
        return true;
    } else if c < '\u{1F18}' {
        return false;
    } else if c >= '\u{1F18}' && c <= '\u{1F1D}' {
        return true;
    } else if c < '\u{1F20}' {
        return false;
    } else if c >= '\u{1F20}' && c <= '\u{1F45}' {
        return true;
    } else if c < '\u{1F48}' {
        return false;
    } else if c >= '\u{1F48}' && c <= '\u{1F4D}' {
        return true;
    } else if c < '\u{1F50}' {
        return false;
    } else if c >= '\u{1F50}' && c <= '\u{1F57}' {
        return true;
    } else if c < '\u{1F59}' {
        return false;
    } else if c >= '\u{1F59}' && c <= '\u{1F59}' {
        return true;
    } else if c < '\u{1F5B}' {
        return false;
    } else if c >= '\u{1F5B}' && c <= '\u{1F5B}' {
        return true;
    } else if c < '\u{1F5D}' {
        return false;
    } else if c >= '\u{1F5D}' && c <= '\u{1F5D}' {
        return true;
    } else if c < '\u{1F5F}' {
        return false;
    } else if c >= '\u{1F5F}' && c <= '\u{1F7D}' {
        return true;
    } else if c < '\u{1F80}' {
        return false;
    } else if c >= '\u{1F80}' && c <= '\u{1FB4}' {
        return true;
    } else if c < '\u{1FB6}' {
        return false;
    } else if c >= '\u{1FB6}' && c <= '\u{1FBC}' {
        return true;
    } else if c < '\u{1FBE}' {
        return false;
    } else if c >= '\u{1FBE}' && c <= '\u{1FBE}' {
        return true;
    } else if c < '\u{1FC2}' {
        return false;
    } else if c >= '\u{1FC2}' && c <= '\u{1FC4}' {
        return true;
    } else if c < '\u{1FC6}' {
        return false;
    } else if c >= '\u{1FC6}' && c <= '\u{1FCC}' {
        return true;
    } else if c < '\u{1FD0}' {
        return false;
    } else if c >= '\u{1FD0}' && c <= '\u{1FD3}' {
        return true;
    } else if c < '\u{1FD6}' {
        return false;
    } else if c >= '\u{1FD6}' && c <= '\u{1FDB}' {
        return true;
    } else if c < '\u{1FE0}' {
        return false;
    } else if c >= '\u{1FE0}' && c <= '\u{1FEC}' {
        return true;
    } else if c < '\u{1FF2}' {
        return false;
    } else if c >= '\u{1FF2}' && c <= '\u{1FF4}' {
        return true;
    } else if c < '\u{1FF6}' {
        return false;
    } else if c >= '\u{1FF6}' && c <= '\u{1FFC}' {
        return true;
    } else if c < '\u{2071}' {
        return false;
    } else if c >= '\u{2071}' && c <= '\u{2071}' {
        return true;
    } else if c < '\u{207F}' {
        return false;
    } else if c >= '\u{207F}' && c <= '\u{207F}' {
        return true;
    } else if c < '\u{2090}' {
        return false;
    } else if c >= '\u{2090}' && c <= '\u{209C}' {
        return true;
    } else if c < '\u{2102}' {
        return false;
    } else if c >= '\u{2102}' && c <= '\u{2102}' {
        return true;
    } else if c < '\u{2107}' {
        return false;
    } else if c >= '\u{2107}' && c <= '\u{2107}' {
        return true;
    } else if c < '\u{210A}' {
        return false;
    } else if c >= '\u{210A}' && c <= '\u{2113}' {
        return true;
    } else if c < '\u{2115}' {
        return false;
    } else if c >= '\u{2115}' && c <= '\u{2115}' {
        return true;
    } else if c < '\u{2118}' {
        return false;
    } else if c >= '\u{2118}' && c <= '\u{211D}' {
        return true;
    } else if c < '\u{2124}' {
        return false;
    } else if c >= '\u{2124}' && c <= '\u{2124}' {
        return true;
    } else if c < '\u{2126}' {
        return false;
    } else if c >= '\u{2126}' && c <= '\u{2126}' {
        return true;
    } else if c < '\u{2128}' {
        return false;
    } else if c >= '\u{2128}' && c <= '\u{2128}' {
        return true;
    } else if c < '\u{212A}' {
        return false;
    } else if c >= '\u{212A}' && c <= '\u{2139}' {
        return true;
    } else if c < '\u{213C}' {
        return false;
    } else if c >= '\u{213C}' && c <= '\u{213F}' {
        return true;
    } else if c < '\u{2145}' {
        return false;
    } else if c >= '\u{2145}' && c <= '\u{2149}' {
        return true;
    } else if c < '\u{214E}' {
        return false;
    } else if c >= '\u{214E}' && c <= '\u{214E}' {
        return true;
    } else if c < '\u{2160}' {
        return false;
    } else if c >= '\u{2160}' && c <= '\u{2188}' {
        return true;
    } else if c < '\u{2C00}' {
        return false;
    } else if c >= '\u{2C00}' && c <= '\u{2C2E}' {
        return true;
    } else if c < '\u{2C30}' {
        return false;
    } else if c >= '\u{2C30}' && c <= '\u{2C5E}' {
        return true;
    } else if c < '\u{2C60}' {
        return false;
    } else if c >= '\u{2C60}' && c <= '\u{2CE4}' {
        return true;
    } else if c < '\u{2CEB}' {
        return false;
    } else if c >= '\u{2CEB}' && c <= '\u{2CEE}' {
        return true;
    } else if c < '\u{2CF2}' {
        return false;
    } else if c >= '\u{2CF2}' && c <= '\u{2CF3}' {
        return true;
    } else if c < '\u{2D00}' {
        return false;
    } else if c >= '\u{2D00}' && c <= '\u{2D25}' {
        return true;
    } else if c < '\u{2D27}' {
        return false;
    } else if c >= '\u{2D27}' && c <= '\u{2D27}' {
        return true;
    } else if c < '\u{2D2D}' {
        return false;
    } else if c >= '\u{2D2D}' && c <= '\u{2D2D}' {
        return true;
    } else if c < '\u{2D30}' {
        return false;
    } else if c >= '\u{2D30}' && c <= '\u{2D67}' {
        return true;
    } else if c < '\u{2D6F}' {
        return false;
    } else if c >= '\u{2D6F}' && c <= '\u{2D6F}' {
        return true;
    } else if c < '\u{2D80}' {
        return false;
    } else if c >= '\u{2D80}' && c <= '\u{2D96}' {
        return true;
    } else if c < '\u{2DA0}' {
        return false;
    } else if c >= '\u{2DA0}' && c <= '\u{2DA6}' {
        return true;
    } else if c < '\u{2DA8}' {
        return false;
    } else if c >= '\u{2DA8}' && c <= '\u{2DAE}' {
        return true;
    } else if c < '\u{2DB0}' {
        return false;
    } else if c >= '\u{2DB0}' && c <= '\u{2DB6}' {
        return true;
    } else if c < '\u{2DB8}' {
        return false;
    } else if c >= '\u{2DB8}' && c <= '\u{2DBE}' {
        return true;
    } else if c < '\u{2DC0}' {
        return false;
    } else if c >= '\u{2DC0}' && c <= '\u{2DC6}' {
        return true;
    } else if c < '\u{2DC8}' {
        return false;
    } else if c >= '\u{2DC8}' && c <= '\u{2DCE}' {
        return true;
    } else if c < '\u{2DD0}' {
        return false;
    } else if c >= '\u{2DD0}' && c <= '\u{2DD6}' {
        return true;
    } else if c < '\u{2DD8}' {
        return false;
    } else if c >= '\u{2DD8}' && c <= '\u{2DDE}' {
        return true;
    } else if c < '\u{3005}' {
        return false;
    } else if c >= '\u{3005}' && c <= '\u{3007}' {
        return true;
    } else if c < '\u{3021}' {
        return false;
    } else if c >= '\u{3021}' && c <= '\u{3029}' {
        return true;
    } else if c < '\u{3031}' {
        return false;
    } else if c >= '\u{3031}' && c <= '\u{3035}' {
        return true;
    } else if c < '\u{3038}' {
        return false;
    } else if c >= '\u{3038}' && c <= '\u{303C}' {
        return true;
    } else if c < '\u{3041}' {
        return false;
    } else if c >= '\u{3041}' && c <= '\u{3096}' {
        return true;
    } else if c < '\u{309B}' {
        return false;
    } else if c >= '\u{309B}' && c <= '\u{309F}' {
        return true;
    } else if c < '\u{30A1}' {
        return false;
    } else if c >= '\u{30A1}' && c <= '\u{30FA}' {
        return true;
    } else if c < '\u{30FC}' {
        return false;
    } else if c >= '\u{30FC}' && c <= '\u{30FF}' {
        return true;
    } else if c < '\u{3105}' {
        return false;
    } else if c >= '\u{3105}' && c <= '\u{312D}' {
        return true;
    } else if c < '\u{3131}' {
        return false;
    } else if c >= '\u{3131}' && c <= '\u{318E}' {
        return true;
    } else if c < '\u{31A0}' {
        return false;
    } else if c >= '\u{31A0}' && c <= '\u{31BA}' {
        return true;
    } else if c < '\u{31F0}' {
        return false;
    } else if c >= '\u{31F0}' && c <= '\u{31FF}' {
        return true;
    } else if c < '\u{3400}' {
        return false;
    } else if c >= '\u{3400}' && c <= '\u{4DB5}' {
        return true;
    } else if c < '\u{4E00}' {
        return false;
    } else if c >= '\u{4E00}' && c <= '\u{9FD5}' {
        return true;
    } else if c < '\u{A000}' {
        return false;
    } else if c >= '\u{A000}' && c <= '\u{A48C}' {
        return true;
    } else if c < '\u{A4D0}' {
        return false;
    } else if c >= '\u{A4D0}' && c <= '\u{A4FD}' {
        return true;
    } else if c < '\u{A500}' {
        return false;
    } else if c >= '\u{A500}' && c <= '\u{A60C}' {
        return true;
    } else if c < '\u{A610}' {
        return false;
    } else if c >= '\u{A610}' && c <= '\u{A61F}' {
        return true;
    } else if c < '\u{A62A}' {
        return false;
    } else if c >= '\u{A62A}' && c <= '\u{A62B}' {
        return true;
    } else if c < '\u{A640}' {
        return false;
    } else if c >= '\u{A640}' && c <= '\u{A66E}' {
        return true;
    } else if c < '\u{A67F}' {
        return false;
    } else if c >= '\u{A67F}' && c <= '\u{A69D}' {
        return true;
    } else if c < '\u{A6A0}' {
        return false;
    } else if c >= '\u{A6A0}' && c <= '\u{A6EF}' {
        return true;
    } else if c < '\u{A717}' {
        return false;
    } else if c >= '\u{A717}' && c <= '\u{A71F}' {
        return true;
    } else if c < '\u{A722}' {
        return false;
    } else if c >= '\u{A722}' && c <= '\u{A788}' {
        return true;
    } else if c < '\u{A78B}' {
        return false;
    } else if c >= '\u{A78B}' && c <= '\u{A7AE}' {
        return true;
    } else if c < '\u{A7B0}' {
        return false;
    } else if c >= '\u{A7B0}' && c <= '\u{A7B7}' {
        return true;
    } else if c < '\u{A7F7}' {
        return false;
    } else if c >= '\u{A7F7}' && c <= '\u{A801}' {
        return true;
    } else if c < '\u{A803}' {
        return false;
    } else if c >= '\u{A803}' && c <= '\u{A805}' {
        return true;
    } else if c < '\u{A807}' {
        return false;
    } else if c >= '\u{A807}' && c <= '\u{A80A}' {
        return true;
    } else if c < '\u{A80C}' {
        return false;
    } else if c >= '\u{A80C}' && c <= '\u{A822}' {
        return true;
    } else if c < '\u{A840}' {
        return false;
    } else if c >= '\u{A840}' && c <= '\u{A873}' {
        return true;
    } else if c < '\u{A882}' {
        return false;
    } else if c >= '\u{A882}' && c <= '\u{A8B3}' {
        return true;
    } else if c < '\u{A8F2}' {
        return false;
    } else if c >= '\u{A8F2}' && c <= '\u{A8F7}' {
        return true;
    } else if c < '\u{A8FB}' {
        return false;
    } else if c >= '\u{A8FB}' && c <= '\u{A8FB}' {
        return true;
    } else if c < '\u{A8FD}' {
        return false;
    } else if c >= '\u{A8FD}' && c <= '\u{A8FD}' {
        return true;
    } else if c < '\u{A90A}' {
        return false;
    } else if c >= '\u{A90A}' && c <= '\u{A925}' {
        return true;
    } else if c < '\u{A930}' {
        return false;
    } else if c >= '\u{A930}' && c <= '\u{A946}' {
        return true;
    } else if c < '\u{A960}' {
        return false;
    } else if c >= '\u{A960}' && c <= '\u{A97C}' {
        return true;
    } else if c < '\u{A984}' {
        return false;
    } else if c >= '\u{A984}' && c <= '\u{A9B2}' {
        return true;
    } else if c < '\u{A9CF}' {
        return false;
    } else if c >= '\u{A9CF}' && c <= '\u{A9CF}' {
        return true;
    } else if c < '\u{A9E0}' {
        return false;
    } else if c >= '\u{A9E0}' && c <= '\u{A9E4}' {
        return true;
    } else if c < '\u{A9E6}' {
        return false;
    } else if c >= '\u{A9E6}' && c <= '\u{A9EF}' {
        return true;
    } else if c < '\u{A9FA}' {
        return false;
    } else if c >= '\u{A9FA}' && c <= '\u{A9FE}' {
        return true;
    } else if c < '\u{AA00}' {
        return false;
    } else if c >= '\u{AA00}' && c <= '\u{AA28}' {
        return true;
    } else if c < '\u{AA40}' {
        return false;
    } else if c >= '\u{AA40}' && c <= '\u{AA42}' {
        return true;
    } else if c < '\u{AA44}' {
        return false;
    } else if c >= '\u{AA44}' && c <= '\u{AA4B}' {
        return true;
    } else if c < '\u{AA60}' {
        return false;
    } else if c >= '\u{AA60}' && c <= '\u{AA76}' {
        return true;
    } else if c < '\u{AA7A}' {
        return false;
    } else if c >= '\u{AA7A}' && c <= '\u{AA7A}' {
        return true;
    } else if c < '\u{AA7E}' {
        return false;
    } else if c >= '\u{AA7E}' && c <= '\u{AAAF}' {
        return true;
    } else if c < '\u{AAB1}' {
        return false;
    } else if c >= '\u{AAB1}' && c <= '\u{AAB1}' {
        return true;
    } else if c < '\u{AAB5}' {
        return false;
    } else if c >= '\u{AAB5}' && c <= '\u{AAB6}' {
        return true;
    } else if c < '\u{AAB9}' {
        return false;
    } else if c >= '\u{AAB9}' && c <= '\u{AABD}' {
        return true;
    } else if c < '\u{AAC0}' {
        return false;
    } else if c >= '\u{AAC0}' && c <= '\u{AAC0}' {
        return true;
    } else if c < '\u{AAC2}' {
        return false;
    } else if c >= '\u{AAC2}' && c <= '\u{AAC2}' {
        return true;
    } else if c < '\u{AADB}' {
        return false;
    } else if c >= '\u{AADB}' && c <= '\u{AADD}' {
        return true;
    } else if c < '\u{AAE0}' {
        return false;
    } else if c >= '\u{AAE0}' && c <= '\u{AAEA}' {
        return true;
    } else if c < '\u{AAF2}' {
        return false;
    } else if c >= '\u{AAF2}' && c <= '\u{AAF4}' {
        return true;
    } else if c < '\u{AB01}' {
        return false;
    } else if c >= '\u{AB01}' && c <= '\u{AB06}' {
        return true;
    } else if c < '\u{AB09}' {
        return false;
    } else if c >= '\u{AB09}' && c <= '\u{AB0E}' {
        return true;
    } else if c < '\u{AB11}' {
        return false;
    } else if c >= '\u{AB11}' && c <= '\u{AB16}' {
        return true;
    } else if c < '\u{AB20}' {
        return false;
    } else if c >= '\u{AB20}' && c <= '\u{AB26}' {
        return true;
    } else if c < '\u{AB28}' {
        return false;
    } else if c >= '\u{AB28}' && c <= '\u{AB2E}' {
        return true;
    } else if c < '\u{AB30}' {
        return false;
    } else if c >= '\u{AB30}' && c <= '\u{AB5A}' {
        return true;
    } else if c < '\u{AB5C}' {
        return false;
    } else if c >= '\u{AB5C}' && c <= '\u{AB65}' {
        return true;
    } else if c < '\u{AB70}' {
        return false;
    } else if c >= '\u{AB70}' && c <= '\u{ABE2}' {
        return true;
    } else if c < '\u{AC00}' {
        return false;
    } else if c >= '\u{AC00}' && c <= '\u{D7A3}' {
        return true;
    } else if c < '\u{D7B0}' {
        return false;
    } else if c >= '\u{D7B0}' && c <= '\u{D7C6}' {
        return true;
    } else if c < '\u{D7CB}' {
        return false;
    } else if c >= '\u{D7CB}' && c <= '\u{D7FB}' {
        return true;
    } else if c < '\u{F900}' {
        return false;
    } else if c >= '\u{F900}' && c <= '\u{FA6D}' {
        return true;
    } else if c < '\u{FA70}' {
        return false;
    } else if c >= '\u{FA70}' && c <= '\u{FAD9}' {
        return true;
    } else if c < '\u{FB00}' {
        return false;
    } else if c >= '\u{FB00}' && c <= '\u{FB06}' {
        return true;
    } else if c < '\u{FB13}' {
        return false;
    } else if c >= '\u{FB13}' && c <= '\u{FB17}' {
        return true;
    } else if c < '\u{FB1D}' {
        return false;
    } else if c >= '\u{FB1D}' && c <= '\u{FB1D}' {
        return true;
    } else if c < '\u{FB1F}' {
        return false;
    } else if c >= '\u{FB1F}' && c <= '\u{FB28}' {
        return true;
    } else if c < '\u{FB2A}' {
        return false;
    } else if c >= '\u{FB2A}' && c <= '\u{FB36}' {
        return true;
    } else if c < '\u{FB38}' {
        return false;
    } else if c >= '\u{FB38}' && c <= '\u{FB3C}' {
        return true;
    } else if c < '\u{FB3E}' {
        return false;
    } else if c >= '\u{FB3E}' && c <= '\u{FB3E}' {
        return true;
    } else if c < '\u{FB40}' {
        return false;
    } else if c >= '\u{FB40}' && c <= '\u{FB41}' {
        return true;
    } else if c < '\u{FB43}' {
        return false;
    } else if c >= '\u{FB43}' && c <= '\u{FB44}' {
        return true;
    } else if c < '\u{FB46}' {
        return false;
    } else if c >= '\u{FB46}' && c <= '\u{FBB1}' {
        return true;
    } else if c < '\u{FBD3}' {
        return false;
    } else if c >= '\u{FBD3}' && c <= '\u{FD3D}' {
        return true;
    } else if c < '\u{FD50}' {
        return false;
    } else if c >= '\u{FD50}' && c <= '\u{FD8F}' {
        return true;
    } else if c < '\u{FD92}' {
        return false;
    } else if c >= '\u{FD92}' && c <= '\u{FDC7}' {
        return true;
    } else if c < '\u{FDF0}' {
        return false;
    } else if c >= '\u{FDF0}' && c <= '\u{FDFB}' {
        return true;
    } else if c < '\u{FE70}' {
        return false;
    } else if c >= '\u{FE70}' && c <= '\u{FE74}' {
        return true;
    } else if c < '\u{FE76}' {
        return false;
    } else if c >= '\u{FE76}' && c <= '\u{FEFC}' {
        return true;
    } else if c < '\u{FF21}' {
        return false;
    } else if c >= '\u{FF21}' && c <= '\u{FF3A}' {
        return true;
    } else if c < '\u{FF41}' {
        return false;
    } else if c >= '\u{FF41}' && c <= '\u{FF5A}' {
        return true;
    } else if c < '\u{FF66}' {
        return false;
    } else if c >= '\u{FF66}' && c <= '\u{FFBE}' {
        return true;
    } else if c < '\u{FFC2}' {
        return false;
    } else if c >= '\u{FFC2}' && c <= '\u{FFC7}' {
        return true;
    } else if c < '\u{FFCA}' {
        return false;
    } else if c >= '\u{FFCA}' && c <= '\u{FFCF}' {
        return true;
    } else if c < '\u{FFD2}' {
        return false;
    } else if c >= '\u{FFD2}' && c <= '\u{FFD7}' {
        return true;
    } else if c < '\u{FFDA}' {
        return false;
    } else if c >= '\u{FFDA}' && c <= '\u{FFDC}' {
        return true;
    } else if c < '\u{10000}' {
        return false;
    } else if c >= '\u{10000}' && c <= '\u{1000B}' {
        return true;
    } else if c < '\u{1000D}' {
        return false;
    } else if c >= '\u{1000D}' && c <= '\u{10026}' {
        return true;
    } else if c < '\u{10028}' {
        return false;
    } else if c >= '\u{10028}' && c <= '\u{1003A}' {
        return true;
    } else if c < '\u{1003C}' {
        return false;
    } else if c >= '\u{1003C}' && c <= '\u{1003D}' {
        return true;
    } else if c < '\u{1003F}' {
        return false;
    } else if c >= '\u{1003F}' && c <= '\u{1004D}' {
        return true;
    } else if c < '\u{10050}' {
        return false;
    } else if c >= '\u{10050}' && c <= '\u{1005D}' {
        return true;
    } else if c < '\u{10080}' {
        return false;
    } else if c >= '\u{10080}' && c <= '\u{100FA}' {
        return true;
    } else if c < '\u{10140}' {
        return false;
    } else if c >= '\u{10140}' && c <= '\u{10174}' {
        return true;
    } else if c < '\u{10280}' {
        return false;
    } else if c >= '\u{10280}' && c <= '\u{1029C}' {
        return true;
    } else if c < '\u{102A0}' {
        return false;
    } else if c >= '\u{102A0}' && c <= '\u{102D0}' {
        return true;
    } else if c < '\u{10300}' {
        return false;
    } else if c >= '\u{10300}' && c <= '\u{1031F}' {
        return true;
    } else if c < '\u{10330}' {
        return false;
    } else if c >= '\u{10330}' && c <= '\u{1034A}' {
        return true;
    } else if c < '\u{10350}' {
        return false;
    } else if c >= '\u{10350}' && c <= '\u{10375}' {
        return true;
    } else if c < '\u{10380}' {
        return false;
    } else if c >= '\u{10380}' && c <= '\u{1039D}' {
        return true;
    } else if c < '\u{103A0}' {
        return false;
    } else if c >= '\u{103A0}' && c <= '\u{103C3}' {
        return true;
    } else if c < '\u{103C8}' {
        return false;
    } else if c >= '\u{103C8}' && c <= '\u{103CF}' {
        return true;
    } else if c < '\u{103D1}' {
        return false;
    } else if c >= '\u{103D1}' && c <= '\u{103D5}' {
        return true;
    } else if c < '\u{10400}' {
        return false;
    } else if c >= '\u{10400}' && c <= '\u{1049D}' {
        return true;
    } else if c < '\u{104B0}' {
        return false;
    } else if c >= '\u{104B0}' && c <= '\u{104D3}' {
        return true;
    } else if c < '\u{104D8}' {
        return false;
    } else if c >= '\u{104D8}' && c <= '\u{104FB}' {
        return true;
    } else if c < '\u{10500}' {
        return false;
    } else if c >= '\u{10500}' && c <= '\u{10527}' {
        return true;
    } else if c < '\u{10530}' {
        return false;
    } else if c >= '\u{10530}' && c <= '\u{10563}' {
        return true;
    } else if c < '\u{10600}' {
        return false;
    } else if c >= '\u{10600}' && c <= '\u{10736}' {
        return true;
    } else if c < '\u{10740}' {
        return false;
    } else if c >= '\u{10740}' && c <= '\u{10755}' {
        return true;
    } else if c < '\u{10760}' {
        return false;
    } else if c >= '\u{10760}' && c <= '\u{10767}' {
        return true;
    } else if c < '\u{10800}' {
        return false;
    } else if c >= '\u{10800}' && c <= '\u{10805}' {
        return true;
    } else if c < '\u{10808}' {
        return false;
    } else if c >= '\u{10808}' && c <= '\u{10808}' {
        return true;
    } else if c < '\u{1080A}' {
        return false;
    } else if c >= '\u{1080A}' && c <= '\u{10835}' {
        return true;
    } else if c < '\u{10837}' {
        return false;
    } else if c >= '\u{10837}' && c <= '\u{10838}' {
        return true;
    } else if c < '\u{1083C}' {
        return false;
    } else if c >= '\u{1083C}' && c <= '\u{1083C}' {
        return true;
    } else if c < '\u{1083F}' {
        return false;
    } else if c >= '\u{1083F}' && c <= '\u{10855}' {
        return true;
    } else if c < '\u{10860}' {
        return false;
    } else if c >= '\u{10860}' && c <= '\u{10876}' {
        return true;
    } else if c < '\u{10880}' {
        return false;
    } else if c >= '\u{10880}' && c <= '\u{1089E}' {
        return true;
    } else if c < '\u{108E0}' {
        return false;
    } else if c >= '\u{108E0}' && c <= '\u{108F2}' {
        return true;
    } else if c < '\u{108F4}' {
        return false;
    } else if c >= '\u{108F4}' && c <= '\u{108F5}' {
        return true;
    } else if c < '\u{10900}' {
        return false;
    } else if c >= '\u{10900}' && c <= '\u{10915}' {
        return true;
    } else if c < '\u{10920}' {
        return false;
    } else if c >= '\u{10920}' && c <= '\u{10939}' {
        return true;
    } else if c < '\u{10980}' {
        return false;
    } else if c >= '\u{10980}' && c <= '\u{109B7}' {
        return true;
    } else if c < '\u{109BE}' {
        return false;
    } else if c >= '\u{109BE}' && c <= '\u{109BF}' {
        return true;
    } else if c < '\u{10A00}' {
        return false;
    } else if c >= '\u{10A00}' && c <= '\u{10A00}' {
        return true;
    } else if c < '\u{10A10}' {
        return false;
    } else if c >= '\u{10A10}' && c <= '\u{10A13}' {
        return true;
    } else if c < '\u{10A15}' {
        return false;
    } else if c >= '\u{10A15}' && c <= '\u{10A17}' {
        return true;
    } else if c < '\u{10A19}' {
        return false;
    } else if c >= '\u{10A19}' && c <= '\u{10A33}' {
        return true;
    } else if c < '\u{10A60}' {
        return false;
    } else if c >= '\u{10A60}' && c <= '\u{10A7C}' {
        return true;
    } else if c < '\u{10A80}' {
        return false;
    } else if c >= '\u{10A80}' && c <= '\u{10A9C}' {
        return true;
    } else if c < '\u{10AC0}' {
        return false;
    } else if c >= '\u{10AC0}' && c <= '\u{10AC7}' {
        return true;
    } else if c < '\u{10AC9}' {
        return false;
    } else if c >= '\u{10AC9}' && c <= '\u{10AE4}' {
        return true;
    } else if c < '\u{10B00}' {
        return false;
    } else if c >= '\u{10B00}' && c <= '\u{10B35}' {
        return true;
    } else if c < '\u{10B40}' {
        return false;
    } else if c >= '\u{10B40}' && c <= '\u{10B55}' {
        return true;
    } else if c < '\u{10B60}' {
        return false;
    } else if c >= '\u{10B60}' && c <= '\u{10B72}' {
        return true;
    } else if c < '\u{10B80}' {
        return false;
    } else if c >= '\u{10B80}' && c <= '\u{10B91}' {
        return true;
    } else if c < '\u{10C00}' {
        return false;
    } else if c >= '\u{10C00}' && c <= '\u{10C48}' {
        return true;
    } else if c < '\u{10C80}' {
        return false;
    } else if c >= '\u{10C80}' && c <= '\u{10CB2}' {
        return true;
    } else if c < '\u{10CC0}' {
        return false;
    } else if c >= '\u{10CC0}' && c <= '\u{10CF2}' {
        return true;
    } else if c < '\u{11003}' {
        return false;
    } else if c >= '\u{11003}' && c <= '\u{11037}' {
        return true;
    } else if c < '\u{11083}' {
        return false;
    } else if c >= '\u{11083}' && c <= '\u{110AF}' {
        return true;
    } else if c < '\u{110D0}' {
        return false;
    } else if c >= '\u{110D0}' && c <= '\u{110E8}' {
        return true;
    } else if c < '\u{11103}' {
        return false;
    } else if c >= '\u{11103}' && c <= '\u{11126}' {
        return true;
    } else if c < '\u{11150}' {
        return false;
    } else if c >= '\u{11150}' && c <= '\u{11172}' {
        return true;
    } else if c < '\u{11176}' {
        return false;
    } else if c >= '\u{11176}' && c <= '\u{11176}' {
        return true;
    } else if c < '\u{11183}' {
        return false;
    } else if c >= '\u{11183}' && c <= '\u{111B2}' {
        return true;
    } else if c < '\u{111C1}' {
        return false;
    } else if c >= '\u{111C1}' && c <= '\u{111C4}' {
        return true;
    } else if c < '\u{111DA}' {
        return false;
    } else if c >= '\u{111DA}' && c <= '\u{111DA}' {
        return true;
    } else if c < '\u{111DC}' {
        return false;
    } else if c >= '\u{111DC}' && c <= '\u{111DC}' {
        return true;
    } else if c < '\u{11200}' {
        return false;
    } else if c >= '\u{11200}' && c <= '\u{11211}' {
        return true;
    } else if c < '\u{11213}' {
        return false;
    } else if c >= '\u{11213}' && c <= '\u{1122B}' {
        return true;
    } else if c < '\u{11280}' {
        return false;
    } else if c >= '\u{11280}' && c <= '\u{11286}' {
        return true;
    } else if c < '\u{11288}' {
        return false;
    } else if c >= '\u{11288}' && c <= '\u{11288}' {
        return true;
    } else if c < '\u{1128A}' {
        return false;
    } else if c >= '\u{1128A}' && c <= '\u{1128D}' {
        return true;
    } else if c < '\u{1128F}' {
        return false;
    } else if c >= '\u{1128F}' && c <= '\u{1129D}' {
        return true;
    } else if c < '\u{1129F}' {
        return false;
    } else if c >= '\u{1129F}' && c <= '\u{112A8}' {
        return true;
    } else if c < '\u{112B0}' {
        return false;
    } else if c >= '\u{112B0}' && c <= '\u{112DE}' {
        return true;
    } else if c < '\u{11305}' {
        return false;
    } else if c >= '\u{11305}' && c <= '\u{1130C}' {
        return true;
    } else if c < '\u{1130F}' {
        return false;
    } else if c >= '\u{1130F}' && c <= '\u{11310}' {
        return true;
    } else if c < '\u{11313}' {
        return false;
    } else if c >= '\u{11313}' && c <= '\u{11328}' {
        return true;
    } else if c < '\u{1132A}' {
        return false;
    } else if c >= '\u{1132A}' && c <= '\u{11330}' {
        return true;
    } else if c < '\u{11332}' {
        return false;
    } else if c >= '\u{11332}' && c <= '\u{11333}' {
        return true;
    } else if c < '\u{11335}' {
        return false;
    } else if c >= '\u{11335}' && c <= '\u{11339}' {
        return true;
    } else if c < '\u{1133D}' {
        return false;
    } else if c >= '\u{1133D}' && c <= '\u{1133D}' {
        return true;
    } else if c < '\u{11350}' {
        return false;
    } else if c >= '\u{11350}' && c <= '\u{11350}' {
        return true;
    } else if c < '\u{1135D}' {
        return false;
    } else if c >= '\u{1135D}' && c <= '\u{11361}' {
        return true;
    } else if c < '\u{11400}' {
        return false;
    } else if c >= '\u{11400}' && c <= '\u{11434}' {
        return true;
    } else if c < '\u{11447}' {
        return false;
    } else if c >= '\u{11447}' && c <= '\u{1144A}' {
        return true;
    } else if c < '\u{11480}' {
        return false;
    } else if c >= '\u{11480}' && c <= '\u{114AF}' {
        return true;
    } else if c < '\u{114C4}' {
        return false;
    } else if c >= '\u{114C4}' && c <= '\u{114C5}' {
        return true;
    } else if c < '\u{114C7}' {
        return false;
    } else if c >= '\u{114C7}' && c <= '\u{114C7}' {
        return true;
    } else if c < '\u{11580}' {
        return false;
    } else if c >= '\u{11580}' && c <= '\u{115AE}' {
        return true;
    } else if c < '\u{115D8}' {
        return false;
    } else if c >= '\u{115D8}' && c <= '\u{115DB}' {
        return true;
    } else if c < '\u{11600}' {
        return false;
    } else if c >= '\u{11600}' && c <= '\u{1162F}' {
        return true;
    } else if c < '\u{11644}' {
        return false;
    } else if c >= '\u{11644}' && c <= '\u{11644}' {
        return true;
    } else if c < '\u{11680}' {
        return false;
    } else if c >= '\u{11680}' && c <= '\u{116AA}' {
        return true;
    } else if c < '\u{11700}' {
        return false;
    } else if c >= '\u{11700}' && c <= '\u{11719}' {
        return true;
    } else if c < '\u{118A0}' {
        return false;
    } else if c >= '\u{118A0}' && c <= '\u{118DF}' {
        return true;
    } else if c < '\u{118FF}' {
        return false;
    } else if c >= '\u{118FF}' && c <= '\u{118FF}' {
        return true;
    } else if c < '\u{11AC0}' {
        return false;
    } else if c >= '\u{11AC0}' && c <= '\u{11AF8}' {
        return true;
    } else if c < '\u{11C00}' {
        return false;
    } else if c >= '\u{11C00}' && c <= '\u{11C08}' {
        return true;
    } else if c < '\u{11C0A}' {
        return false;
    } else if c >= '\u{11C0A}' && c <= '\u{11C2E}' {
        return true;
    } else if c < '\u{11C40}' {
        return false;
    } else if c >= '\u{11C40}' && c <= '\u{11C40}' {
        return true;
    } else if c < '\u{11C72}' {
        return false;
    } else if c >= '\u{11C72}' && c <= '\u{11C8F}' {
        return true;
    } else if c < '\u{12000}' {
        return false;
    } else if c >= '\u{12000}' && c <= '\u{12399}' {
        return true;
    } else if c < '\u{12400}' {
        return false;
    } else if c >= '\u{12400}' && c <= '\u{1246E}' {
        return true;
    } else if c < '\u{12480}' {
        return false;
    } else if c >= '\u{12480}' && c <= '\u{12543}' {
        return true;
    } else if c < '\u{13000}' {
        return false;
    } else if c >= '\u{13000}' && c <= '\u{1342E}' {
        return true;
    } else if c < '\u{14400}' {
        return false;
    } else if c >= '\u{14400}' && c <= '\u{14646}' {
        return true;
    } else if c < '\u{16800}' {
        return false;
    } else if c >= '\u{16800}' && c <= '\u{16A38}' {
        return true;
    } else if c < '\u{16A40}' {
        return false;
    } else if c >= '\u{16A40}' && c <= '\u{16A5E}' {
        return true;
    } else if c < '\u{16AD0}' {
        return false;
    } else if c >= '\u{16AD0}' && c <= '\u{16AED}' {
        return true;
    } else if c < '\u{16B00}' {
        return false;
    } else if c >= '\u{16B00}' && c <= '\u{16B2F}' {
        return true;
    } else if c < '\u{16B40}' {
        return false;
    } else if c >= '\u{16B40}' && c <= '\u{16B43}' {
        return true;
    } else if c < '\u{16B63}' {
        return false;
    } else if c >= '\u{16B63}' && c <= '\u{16B77}' {
        return true;
    } else if c < '\u{16B7D}' {
        return false;
    } else if c >= '\u{16B7D}' && c <= '\u{16B8F}' {
        return true;
    } else if c < '\u{16F00}' {
        return false;
    } else if c >= '\u{16F00}' && c <= '\u{16F44}' {
        return true;
    } else if c < '\u{16F50}' {
        return false;
    } else if c >= '\u{16F50}' && c <= '\u{16F50}' {
        return true;
    } else if c < '\u{16F93}' {
        return false;
    } else if c >= '\u{16F93}' && c <= '\u{16F9F}' {
        return true;
    } else if c < '\u{16FE0}' {
        return false;
    } else if c >= '\u{16FE0}' && c <= '\u{16FE0}' {
        return true;
    } else if c < '\u{17000}' {
        return false;
    } else if c >= '\u{17000}' && c <= '\u{187EC}' {
        return true;
    } else if c < '\u{18800}' {
        return false;
    } else if c >= '\u{18800}' && c <= '\u{18AF2}' {
        return true;
    } else if c < '\u{1B000}' {
        return false;
    } else if c >= '\u{1B000}' && c <= '\u{1B001}' {
        return true;
    } else if c < '\u{1BC00}' {
        return false;
    } else if c >= '\u{1BC00}' && c <= '\u{1BC6A}' {
        return true;
    } else if c < '\u{1BC70}' {
        return false;
    } else if c >= '\u{1BC70}' && c <= '\u{1BC7C}' {
        return true;
    } else if c < '\u{1BC80}' {
        return false;
    } else if c >= '\u{1BC80}' && c <= '\u{1BC88}' {
        return true;
    } else if c < '\u{1BC90}' {
        return false;
    } else if c >= '\u{1BC90}' && c <= '\u{1BC99}' {
        return true;
    } else if c < '\u{1D400}' {
        return false;
    } else if c >= '\u{1D400}' && c <= '\u{1D454}' {
        return true;
    } else if c < '\u{1D456}' {
        return false;
    } else if c >= '\u{1D456}' && c <= '\u{1D49C}' {
        return true;
    } else if c < '\u{1D49E}' {
        return false;
    } else if c >= '\u{1D49E}' && c <= '\u{1D49F}' {
        return true;
    } else if c < '\u{1D4A2}' {
        return false;
    } else if c >= '\u{1D4A2}' && c <= '\u{1D4A2}' {
        return true;
    } else if c < '\u{1D4A5}' {
        return false;
    } else if c >= '\u{1D4A5}' && c <= '\u{1D4A6}' {
        return true;
    } else if c < '\u{1D4A9}' {
        return false;
    } else if c >= '\u{1D4A9}' && c <= '\u{1D4AC}' {
        return true;
    } else if c < '\u{1D4AE}' {
        return false;
    } else if c >= '\u{1D4AE}' && c <= '\u{1D4B9}' {
        return true;
    } else if c < '\u{1D4BB}' {
        return false;
    } else if c >= '\u{1D4BB}' && c <= '\u{1D4BB}' {
        return true;
    } else if c < '\u{1D4BD}' {
        return false;
    } else if c >= '\u{1D4BD}' && c <= '\u{1D4C3}' {
        return true;
    } else if c < '\u{1D4C5}' {
        return false;
    } else if c >= '\u{1D4C5}' && c <= '\u{1D505}' {
        return true;
    } else if c < '\u{1D507}' {
        return false;
    } else if c >= '\u{1D507}' && c <= '\u{1D50A}' {
        return true;
    } else if c < '\u{1D50D}' {
        return false;
    } else if c >= '\u{1D50D}' && c <= '\u{1D514}' {
        return true;
    } else if c < '\u{1D516}' {
        return false;
    } else if c >= '\u{1D516}' && c <= '\u{1D51C}' {
        return true;
    } else if c < '\u{1D51E}' {
        return false;
    } else if c >= '\u{1D51E}' && c <= '\u{1D539}' {
        return true;
    } else if c < '\u{1D53B}' {
        return false;
    } else if c >= '\u{1D53B}' && c <= '\u{1D53E}' {
        return true;
    } else if c < '\u{1D540}' {
        return false;
    } else if c >= '\u{1D540}' && c <= '\u{1D544}' {
        return true;
    } else if c < '\u{1D546}' {
        return false;
    } else if c >= '\u{1D546}' && c <= '\u{1D546}' {
        return true;
    } else if c < '\u{1D54A}' {
        return false;
    } else if c >= '\u{1D54A}' && c <= '\u{1D550}' {
        return true;
    } else if c < '\u{1D552}' {
        return false;
    } else if c >= '\u{1D552}' && c <= '\u{1D6A5}' {
        return true;
    } else if c < '\u{1D6A8}' {
        return false;
    } else if c >= '\u{1D6A8}' && c <= '\u{1D6C0}' {
        return true;
    } else if c < '\u{1D6C2}' {
        return false;
    } else if c >= '\u{1D6C2}' && c <= '\u{1D6DA}' {
        return true;
    } else if c < '\u{1D6DC}' {
        return false;
    } else if c >= '\u{1D6DC}' && c <= '\u{1D6FA}' {
        return true;
    } else if c < '\u{1D6FC}' {
        return false;
    } else if c >= '\u{1D6FC}' && c <= '\u{1D714}' {
        return true;
    } else if c < '\u{1D716}' {
        return false;
    } else if c >= '\u{1D716}' && c <= '\u{1D734}' {
        return true;
    } else if c < '\u{1D736}' {
        return false;
    } else if c >= '\u{1D736}' && c <= '\u{1D74E}' {
        return true;
    } else if c < '\u{1D750}' {
        return false;
    } else if c >= '\u{1D750}' && c <= '\u{1D76E}' {
        return true;
    } else if c < '\u{1D770}' {
        return false;
    } else if c >= '\u{1D770}' && c <= '\u{1D788}' {
        return true;
    } else if c < '\u{1D78A}' {
        return false;
    } else if c >= '\u{1D78A}' && c <= '\u{1D7A8}' {
        return true;
    } else if c < '\u{1D7AA}' {
        return false;
    } else if c >= '\u{1D7AA}' && c <= '\u{1D7C2}' {
        return true;
    } else if c < '\u{1D7C4}' {
        return false;
    } else if c >= '\u{1D7C4}' && c <= '\u{1D7CB}' {
        return true;
    } else if c < '\u{1E800}' {
        return false;
    } else if c >= '\u{1E800}' && c <= '\u{1E8C4}' {
        return true;
    } else if c < '\u{1E900}' {
        return false;
    } else if c >= '\u{1E900}' && c <= '\u{1E943}' {
        return true;
    } else if c < '\u{1EE00}' {
        return false;
    } else if c >= '\u{1EE00}' && c <= '\u{1EE03}' {
        return true;
    } else if c < '\u{1EE05}' {
        return false;
    } else if c >= '\u{1EE05}' && c <= '\u{1EE1F}' {
        return true;
    } else if c < '\u{1EE21}' {
        return false;
    } else if c >= '\u{1EE21}' && c <= '\u{1EE22}' {
        return true;
    } else if c < '\u{1EE24}' {
        return false;
    } else if c >= '\u{1EE24}' && c <= '\u{1EE24}' {
        return true;
    } else if c < '\u{1EE27}' {
        return false;
    } else if c >= '\u{1EE27}' && c <= '\u{1EE27}' {
        return true;
    } else if c < '\u{1EE29}' {
        return false;
    } else if c >= '\u{1EE29}' && c <= '\u{1EE32}' {
        return true;
    } else if c < '\u{1EE34}' {
        return false;
    } else if c >= '\u{1EE34}' && c <= '\u{1EE37}' {
        return true;
    } else if c < '\u{1EE39}' {
        return false;
    } else if c >= '\u{1EE39}' && c <= '\u{1EE39}' {
        return true;
    } else if c < '\u{1EE3B}' {
        return false;
    } else if c >= '\u{1EE3B}' && c <= '\u{1EE3B}' {
        return true;
    } else if c < '\u{1EE42}' {
        return false;
    } else if c >= '\u{1EE42}' && c <= '\u{1EE42}' {
        return true;
    } else if c < '\u{1EE47}' {
        return false;
    } else if c >= '\u{1EE47}' && c <= '\u{1EE47}' {
        return true;
    } else if c < '\u{1EE49}' {
        return false;
    } else if c >= '\u{1EE49}' && c <= '\u{1EE49}' {
        return true;
    } else if c < '\u{1EE4B}' {
        return false;
    } else if c >= '\u{1EE4B}' && c <= '\u{1EE4B}' {
        return true;
    } else if c < '\u{1EE4D}' {
        return false;
    } else if c >= '\u{1EE4D}' && c <= '\u{1EE4F}' {
        return true;
    } else if c < '\u{1EE51}' {
        return false;
    } else if c >= '\u{1EE51}' && c <= '\u{1EE52}' {
        return true;
    } else if c < '\u{1EE54}' {
        return false;
    } else if c >= '\u{1EE54}' && c <= '\u{1EE54}' {
        return true;
    } else if c < '\u{1EE57}' {
        return false;
    } else if c >= '\u{1EE57}' && c <= '\u{1EE57}' {
        return true;
    } else if c < '\u{1EE59}' {
        return false;
    } else if c >= '\u{1EE59}' && c <= '\u{1EE59}' {
        return true;
    } else if c < '\u{1EE5B}' {
        return false;
    } else if c >= '\u{1EE5B}' && c <= '\u{1EE5B}' {
        return true;
    } else if c < '\u{1EE5D}' {
        return false;
    } else if c >= '\u{1EE5D}' && c <= '\u{1EE5D}' {
        return true;
    } else if c < '\u{1EE5F}' {
        return false;
    } else if c >= '\u{1EE5F}' && c <= '\u{1EE5F}' {
        return true;
    } else if c < '\u{1EE61}' {
        return false;
    } else if c >= '\u{1EE61}' && c <= '\u{1EE62}' {
        return true;
    } else if c < '\u{1EE64}' {
        return false;
    } else if c >= '\u{1EE64}' && c <= '\u{1EE64}' {
        return true;
    } else if c < '\u{1EE67}' {
        return false;
    } else if c >= '\u{1EE67}' && c <= '\u{1EE6A}' {
        return true;
    } else if c < '\u{1EE6C}' {
        return false;
    } else if c >= '\u{1EE6C}' && c <= '\u{1EE72}' {
        return true;
    } else if c < '\u{1EE74}' {
        return false;
    } else if c >= '\u{1EE74}' && c <= '\u{1EE77}' {
        return true;
    } else if c < '\u{1EE79}' {
        return false;
    } else if c >= '\u{1EE79}' && c <= '\u{1EE7C}' {
        return true;
    } else if c < '\u{1EE7E}' {
        return false;
    } else if c >= '\u{1EE7E}' && c <= '\u{1EE7E}' {
        return true;
    } else if c < '\u{1EE80}' {
        return false;
    } else if c >= '\u{1EE80}' && c <= '\u{1EE89}' {
        return true;
    } else if c < '\u{1EE8B}' {
        return false;
    } else if c >= '\u{1EE8B}' && c <= '\u{1EE9B}' {
        return true;
    } else if c < '\u{1EEA1}' {
        return false;
    } else if c >= '\u{1EEA1}' && c <= '\u{1EEA3}' {
        return true;
    } else if c < '\u{1EEA5}' {
        return false;
    } else if c >= '\u{1EEA5}' && c <= '\u{1EEA9}' {
        return true;
    } else if c < '\u{1EEAB}' {
        return false;
    } else if c >= '\u{1EEAB}' && c <= '\u{1EEBB}' {
        return true;
    } else if c < '\u{20000}' {
        return false;
    } else if c >= '\u{20000}' && c <= '\u{2A6D6}' {
        return true;
    } else if c < '\u{2A700}' {
        return false;
    } else if c >= '\u{2A700}' && c <= '\u{2B734}' {
        return true;
    } else if c < '\u{2B740}' {
        return false;
    } else if c >= '\u{2B740}' && c <= '\u{2B81D}' {
        return true;
    } else if c < '\u{2B820}' {
        return false;
    } else if c >= '\u{2B820}' && c <= '\u{2CEA1}' {
        return true;
    } else if c < '\u{2F800}' {
        return false;
    } else if c >= '\u{2F800}' && c <= '\u{2FA1D}' {
        return true;
    }
    false
}

#[inline]
pub(crate) fn is_id_continue(c: char) -> bool {
    if c < '\u{30}' {
        return false;
    } else if c >= '\u{30}' && c <= '\u{39}' {
        return true;
    } else if c < '\u{41}' {
        return false;
    } else if c >= '\u{41}' && c <= '\u{5A}' {
        return true;
    } else if c < '\u{5F}' {
        return false;
    } else if c >= '\u{5F}' && c <= '\u{5F}' {
        return true;
    } else if c < '\u{61}' {
        return false;
    } else if c >= '\u{61}' && c <= '\u{7A}' {
        return true;
    } else if c < '\u{AA}' {
        return false;
    } else if c >= '\u{AA}' && c <= '\u{AA}' {
        return true;
    } else if c < '\u{B5}' {
        return false;
    } else if c >= '\u{B5}' && c <= '\u{B5}' {
        return true;
    } else if c < '\u{B7}' {
        return false;
    } else if c >= '\u{B7}' && c <= '\u{B7}' {
        return true;
    } else if c < '\u{BA}' {
        return false;
    } else if c >= '\u{BA}' && c <= '\u{BA}' {
        return true;
    } else if c < '\u{C0}' {
        return false;
    } else if c >= '\u{C0}' && c <= '\u{D6}' {
        return true;
    } else if c < '\u{D8}' {
        return false;
    } else if c >= '\u{D8}' && c <= '\u{F6}' {
        return true;
    } else if c < '\u{F8}' {
        return false;
    } else if c >= '\u{F8}' && c <= '\u{2C1}' {
        return true;
    } else if c < '\u{2C6}' {
        return false;
    } else if c >= '\u{2C6}' && c <= '\u{2D1}' {
        return true;
    } else if c < '\u{2E0}' {
        return false;
    } else if c >= '\u{2E0}' && c <= '\u{2E4}' {
        return true;
    } else if c < '\u{2EC}' {
        return false;
    } else if c >= '\u{2EC}' && c <= '\u{2EC}' {
        return true;
    } else if c < '\u{2EE}' {
        return false;
    } else if c >= '\u{2EE}' && c <= '\u{2EE}' {
        return true;
    } else if c < '\u{300}' {
        return false;
    } else if c >= '\u{300}' && c <= '\u{374}' {
        return true;
    } else if c < '\u{376}' {
        return false;
    } else if c >= '\u{376}' && c <= '\u{377}' {
        return true;
    } else if c < '\u{37A}' {
        return false;
    } else if c >= '\u{37A}' && c <= '\u{37D}' {
        return true;
    } else if c < '\u{37F}' {
        return false;
    } else if c >= '\u{37F}' && c <= '\u{37F}' {
        return true;
    } else if c < '\u{386}' {
        return false;
    } else if c >= '\u{386}' && c <= '\u{38A}' {
        return true;
    } else if c < '\u{38C}' {
        return false;
    } else if c >= '\u{38C}' && c <= '\u{38C}' {
        return true;
    } else if c < '\u{38E}' {
        return false;
    } else if c >= '\u{38E}' && c <= '\u{3A1}' {
        return true;
    } else if c < '\u{3A3}' {
        return false;
    } else if c >= '\u{3A3}' && c <= '\u{3F5}' {
        return true;
    } else if c < '\u{3F7}' {
        return false;
    } else if c >= '\u{3F7}' && c <= '\u{481}' {
        return true;
    } else if c < '\u{483}' {
        return false;
    } else if c >= '\u{483}' && c <= '\u{487}' {
        return true;
    } else if c < '\u{48A}' {
        return false;
    } else if c >= '\u{48A}' && c <= '\u{52F}' {
        return true;
    } else if c < '\u{531}' {
        return false;
    } else if c >= '\u{531}' && c <= '\u{556}' {
        return true;
    } else if c < '\u{559}' {
        return false;
    } else if c >= '\u{559}' && c <= '\u{559}' {
        return true;
    } else if c < '\u{561}' {
        return false;
    } else if c >= '\u{561}' && c <= '\u{587}' {
        return true;
    } else if c < '\u{591}' {
        return false;
    } else if c >= '\u{591}' && c <= '\u{5BD}' {
        return true;
    } else if c < '\u{5BF}' {
        return false;
    } else if c >= '\u{5BF}' && c <= '\u{5BF}' {
        return true;
    } else if c < '\u{5C1}' {
        return false;
    } else if c >= '\u{5C1}' && c <= '\u{5C2}' {
        return true;
    } else if c < '\u{5C4}' {
        return false;
    } else if c >= '\u{5C4}' && c <= '\u{5C5}' {
        return true;
    } else if c < '\u{5C7}' {
        return false;
    } else if c >= '\u{5C7}' && c <= '\u{5C7}' {
        return true;
    } else if c < '\u{5D0}' {
        return false;
    } else if c >= '\u{5D0}' && c <= '\u{5EA}' {
        return true;
    } else if c < '\u{5F0}' {
        return false;
    } else if c >= '\u{5F0}' && c <= '\u{5F2}' {
        return true;
    } else if c < '\u{610}' {
        return false;
    } else if c >= '\u{610}' && c <= '\u{61A}' {
        return true;
    } else if c < '\u{620}' {
        return false;
    } else if c >= '\u{620}' && c <= '\u{669}' {
        return true;
    } else if c < '\u{66E}' {
        return false;
    } else if c >= '\u{66E}' && c <= '\u{6D3}' {
        return true;
    } else if c < '\u{6D5}' {
        return false;
    } else if c >= '\u{6D5}' && c <= '\u{6DC}' {
        return true;
    } else if c < '\u{6DF}' {
        return false;
    } else if c >= '\u{6DF}' && c <= '\u{6E8}' {
        return true;
    } else if c < '\u{6EA}' {
        return false;
    } else if c >= '\u{6EA}' && c <= '\u{6FC}' {
        return true;
    } else if c < '\u{6FF}' {
        return false;
    } else if c >= '\u{6FF}' && c <= '\u{6FF}' {
        return true;
    } else if c < '\u{710}' {
        return false;
    } else if c >= '\u{710}' && c <= '\u{74A}' {
        return true;
    } else if c < '\u{74D}' {
        return false;
    } else if c >= '\u{74D}' && c <= '\u{7B1}' {
        return true;
    } else if c < '\u{7C0}' {
        return false;
    } else if c >= '\u{7C0}' && c <= '\u{7F5}' {
        return true;
    } else if c < '\u{7FA}' {
        return false;
    } else if c >= '\u{7FA}' && c <= '\u{7FA}' {
        return true;
    } else if c < '\u{800}' {
        return false;
    } else if c >= '\u{800}' && c <= '\u{82D}' {
        return true;
    } else if c < '\u{840}' {
        return false;
    } else if c >= '\u{840}' && c <= '\u{85B}' {
        return true;
    } else if c < '\u{8A0}' {
        return false;
    } else if c >= '\u{8A0}' && c <= '\u{8B4}' {
        return true;
    } else if c < '\u{8B6}' {
        return false;
    } else if c >= '\u{8B6}' && c <= '\u{8BD}' {
        return true;
    } else if c < '\u{8D4}' {
        return false;
    } else if c >= '\u{8D4}' && c <= '\u{8E1}' {
        return true;
    } else if c < '\u{8E3}' {
        return false;
    } else if c >= '\u{8E3}' && c <= '\u{963}' {
        return true;
    } else if c < '\u{966}' {
        return false;
    } else if c >= '\u{966}' && c <= '\u{96F}' {
        return true;
    } else if c < '\u{971}' {
        return false;
    } else if c >= '\u{971}' && c <= '\u{983}' {
        return true;
    } else if c < '\u{985}' {
        return false;
    } else if c >= '\u{985}' && c <= '\u{98C}' {
        return true;
    } else if c < '\u{98F}' {
        return false;
    } else if c >= '\u{98F}' && c <= '\u{990}' {
        return true;
    } else if c < '\u{993}' {
        return false;
    } else if c >= '\u{993}' && c <= '\u{9A8}' {
        return true;
    } else if c < '\u{9AA}' {
        return false;
    } else if c >= '\u{9AA}' && c <= '\u{9B0}' {
        return true;
    } else if c < '\u{9B2}' {
        return false;
    } else if c >= '\u{9B2}' && c <= '\u{9B2}' {
        return true;
    } else if c < '\u{9B6}' {
        return false;
    } else if c >= '\u{9B6}' && c <= '\u{9B9}' {
        return true;
    } else if c < '\u{9BC}' {
        return false;
    } else if c >= '\u{9BC}' && c <= '\u{9C4}' {
        return true;
    } else if c < '\u{9C7}' {
        return false;
    } else if c >= '\u{9C7}' && c <= '\u{9C8}' {
        return true;
    } else if c < '\u{9CB}' {
        return false;
    } else if c >= '\u{9CB}' && c <= '\u{9CE}' {
        return true;
    } else if c < '\u{9D7}' {
        return false;
    } else if c >= '\u{9D7}' && c <= '\u{9D7}' {
        return true;
    } else if c < '\u{9DC}' {
        return false;
    } else if c >= '\u{9DC}' && c <= '\u{9DD}' {
        return true;
    } else if c < '\u{9DF}' {
        return false;
    } else if c >= '\u{9DF}' && c <= '\u{9E3}' {
        return true;
    } else if c < '\u{9E6}' {
        return false;
    } else if c >= '\u{9E6}' && c <= '\u{9F1}' {
        return true;
    } else if c < '\u{A01}' {
        return false;
    } else if c >= '\u{A01}' && c <= '\u{A03}' {
        return true;
    } else if c < '\u{A05}' {
        return false;
    } else if c >= '\u{A05}' && c <= '\u{A0A}' {
        return true;
    } else if c < '\u{A0F}' {
        return false;
    } else if c >= '\u{A0F}' && c <= '\u{A10}' {
        return true;
    } else if c < '\u{A13}' {
        return false;
    } else if c >= '\u{A13}' && c <= '\u{A28}' {
        return true;
    } else if c < '\u{A2A}' {
        return false;
    } else if c >= '\u{A2A}' && c <= '\u{A30}' {
        return true;
    } else if c < '\u{A32}' {
        return false;
    } else if c >= '\u{A32}' && c <= '\u{A33}' {
        return true;
    } else if c < '\u{A35}' {
        return false;
    } else if c >= '\u{A35}' && c <= '\u{A36}' {
        return true;
    } else if c < '\u{A38}' {
        return false;
    } else if c >= '\u{A38}' && c <= '\u{A39}' {
        return true;
    } else if c < '\u{A3C}' {
        return false;
    } else if c >= '\u{A3C}' && c <= '\u{A3C}' {
        return true;
    } else if c < '\u{A3E}' {
        return false;
    } else if c >= '\u{A3E}' && c <= '\u{A42}' {
        return true;
    } else if c < '\u{A47}' {
        return false;
    } else if c >= '\u{A47}' && c <= '\u{A48}' {
        return true;
    } else if c < '\u{A4B}' {
        return false;
    } else if c >= '\u{A4B}' && c <= '\u{A4D}' {
        return true;
    } else if c < '\u{A51}' {
        return false;
    } else if c >= '\u{A51}' && c <= '\u{A51}' {
        return true;
    } else if c < '\u{A59}' {
        return false;
    } else if c >= '\u{A59}' && c <= '\u{A5C}' {
        return true;
    } else if c < '\u{A5E}' {
        return false;
    } else if c >= '\u{A5E}' && c <= '\u{A5E}' {
        return true;
    } else if c < '\u{A66}' {
        return false;
    } else if c >= '\u{A66}' && c <= '\u{A75}' {
        return true;
    } else if c < '\u{A81}' {
        return false;
    } else if c >= '\u{A81}' && c <= '\u{A83}' {
        return true;
    } else if c < '\u{A85}' {
        return false;
    } else if c >= '\u{A85}' && c <= '\u{A8D}' {
        return true;
    } else if c < '\u{A8F}' {
        return false;
    } else if c >= '\u{A8F}' && c <= '\u{A91}' {
        return true;
    } else if c < '\u{A93}' {
        return false;
    } else if c >= '\u{A93}' && c <= '\u{AA8}' {
        return true;
    } else if c < '\u{AAA}' {
        return false;
    } else if c >= '\u{AAA}' && c <= '\u{AB0}' {
        return true;
    } else if c < '\u{AB2}' {
        return false;
    } else if c >= '\u{AB2}' && c <= '\u{AB3}' {
        return true;
    } else if c < '\u{AB5}' {
        return false;
    } else if c >= '\u{AB5}' && c <= '\u{AB9}' {
        return true;
    } else if c < '\u{ABC}' {
        return false;
    } else if c >= '\u{ABC}' && c <= '\u{AC5}' {
        return true;
    } else if c < '\u{AC7}' {
        return false;
    } else if c >= '\u{AC7}' && c <= '\u{AC9}' {
        return true;
    } else if c < '\u{ACB}' {
        return false;
    } else if c >= '\u{ACB}' && c <= '\u{ACD}' {
        return true;
    } else if c < '\u{AD0}' {
        return false;
    } else if c >= '\u{AD0}' && c <= '\u{AD0}' {
        return true;
    } else if c < '\u{AE0}' {
        return false;
    } else if c >= '\u{AE0}' && c <= '\u{AE3}' {
        return true;
    } else if c < '\u{AE6}' {
        return false;
    } else if c >= '\u{AE6}' && c <= '\u{AEF}' {
        return true;
    } else if c < '\u{AF9}' {
        return false;
    } else if c >= '\u{AF9}' && c <= '\u{AF9}' {
        return true;
    } else if c < '\u{B01}' {
        return false;
    } else if c >= '\u{B01}' && c <= '\u{B03}' {
        return true;
    } else if c < '\u{B05}' {
        return false;
    } else if c >= '\u{B05}' && c <= '\u{B0C}' {
        return true;
    } else if c < '\u{B0F}' {
        return false;
    } else if c >= '\u{B0F}' && c <= '\u{B10}' {
        return true;
    } else if c < '\u{B13}' {
        return false;
    } else if c >= '\u{B13}' && c <= '\u{B28}' {
        return true;
    } else if c < '\u{B2A}' {
        return false;
    } else if c >= '\u{B2A}' && c <= '\u{B30}' {
        return true;
    } else if c < '\u{B32}' {
        return false;
    } else if c >= '\u{B32}' && c <= '\u{B33}' {
        return true;
    } else if c < '\u{B35}' {
        return false;
    } else if c >= '\u{B35}' && c <= '\u{B39}' {
        return true;
    } else if c < '\u{B3C}' {
        return false;
    } else if c >= '\u{B3C}' && c <= '\u{B44}' {
        return true;
    } else if c < '\u{B47}' {
        return false;
    } else if c >= '\u{B47}' && c <= '\u{B48}' {
        return true;
    } else if c < '\u{B4B}' {
        return false;
    } else if c >= '\u{B4B}' && c <= '\u{B4D}' {
        return true;
    } else if c < '\u{B56}' {
        return false;
    } else if c >= '\u{B56}' && c <= '\u{B57}' {
        return true;
    } else if c < '\u{B5C}' {
        return false;
    } else if c >= '\u{B5C}' && c <= '\u{B5D}' {
        return true;
    } else if c < '\u{B5F}' {
        return false;
    } else if c >= '\u{B5F}' && c <= '\u{B63}' {
        return true;
    } else if c < '\u{B66}' {
        return false;
    } else if c >= '\u{B66}' && c <= '\u{B6F}' {
        return true;
    } else if c < '\u{B71}' {
        return false;
    } else if c >= '\u{B71}' && c <= '\u{B71}' {
        return true;
    } else if c < '\u{B82}' {
        return false;
    } else if c >= '\u{B82}' && c <= '\u{B83}' {
        return true;
    } else if c < '\u{B85}' {
        return false;
    } else if c >= '\u{B85}' && c <= '\u{B8A}' {
        return true;
    } else if c < '\u{B8E}' {
        return false;
    } else if c >= '\u{B8E}' && c <= '\u{B90}' {
        return true;
    } else if c < '\u{B92}' {
        return false;
    } else if c >= '\u{B92}' && c <= '\u{B95}' {
        return true;
    } else if c < '\u{B99}' {
        return false;
    } else if c >= '\u{B99}' && c <= '\u{B9A}' {
        return true;
    } else if c < '\u{B9C}' {
        return false;
    } else if c >= '\u{B9C}' && c <= '\u{B9C}' {
        return true;
    } else if c < '\u{B9E}' {
        return false;
    } else if c >= '\u{B9E}' && c <= '\u{B9F}' {
        return true;
    } else if c < '\u{BA3}' {
        return false;
    } else if c >= '\u{BA3}' && c <= '\u{BA4}' {
        return true;
    } else if c < '\u{BA8}' {
        return false;
    } else if c >= '\u{BA8}' && c <= '\u{BAA}' {
        return true;
    } else if c < '\u{BAE}' {
        return false;
    } else if c >= '\u{BAE}' && c <= '\u{BB9}' {
        return true;
    } else if c < '\u{BBE}' {
        return false;
    } else if c >= '\u{BBE}' && c <= '\u{BC2}' {
        return true;
    } else if c < '\u{BC6}' {
        return false;
    } else if c >= '\u{BC6}' && c <= '\u{BC8}' {
        return true;
    } else if c < '\u{BCA}' {
        return false;
    } else if c >= '\u{BCA}' && c <= '\u{BCD}' {
        return true;
    } else if c < '\u{BD0}' {
        return false;
    } else if c >= '\u{BD0}' && c <= '\u{BD0}' {
        return true;
    } else if c < '\u{BD7}' {
        return false;
    } else if c >= '\u{BD7}' && c <= '\u{BD7}' {
        return true;
    } else if c < '\u{BE6}' {
        return false;
    } else if c >= '\u{BE6}' && c <= '\u{BEF}' {
        return true;
    } else if c < '\u{C00}' {
        return false;
    } else if c >= '\u{C00}' && c <= '\u{C03}' {
        return true;
    } else if c < '\u{C05}' {
        return false;
    } else if c >= '\u{C05}' && c <= '\u{C0C}' {
        return true;
    } else if c < '\u{C0E}' {
        return false;
    } else if c >= '\u{C0E}' && c <= '\u{C10}' {
        return true;
    } else if c < '\u{C12}' {
        return false;
    } else if c >= '\u{C12}' && c <= '\u{C28}' {
        return true;
    } else if c < '\u{C2A}' {
        return false;
    } else if c >= '\u{C2A}' && c <= '\u{C39}' {
        return true;
    } else if c < '\u{C3D}' {
        return false;
    } else if c >= '\u{C3D}' && c <= '\u{C44}' {
        return true;
    } else if c < '\u{C46}' {
        return false;
    } else if c >= '\u{C46}' && c <= '\u{C48}' {
        return true;
    } else if c < '\u{C4A}' {
        return false;
    } else if c >= '\u{C4A}' && c <= '\u{C4D}' {
        return true;
    } else if c < '\u{C55}' {
        return false;
    } else if c >= '\u{C55}' && c <= '\u{C56}' {
        return true;
    } else if c < '\u{C58}' {
        return false;
    } else if c >= '\u{C58}' && c <= '\u{C5A}' {
        return true;
    } else if c < '\u{C60}' {
        return false;
    } else if c >= '\u{C60}' && c <= '\u{C63}' {
        return true;
    } else if c < '\u{C66}' {
        return false;
    } else if c >= '\u{C66}' && c <= '\u{C6F}' {
        return true;
    } else if c < '\u{C80}' {
        return false;
    } else if c >= '\u{C80}' && c <= '\u{C83}' {
        return true;
    } else if c < '\u{C85}' {
        return false;
    } else if c >= '\u{C85}' && c <= '\u{C8C}' {
        return true;
    } else if c < '\u{C8E}' {
        return false;
    } else if c >= '\u{C8E}' && c <= '\u{C90}' {
        return true;
    } else if c < '\u{C92}' {
        return false;
    } else if c >= '\u{C92}' && c <= '\u{CA8}' {
        return true;
    } else if c < '\u{CAA}' {
        return false;
    } else if c >= '\u{CAA}' && c <= '\u{CB3}' {
        return true;
    } else if c < '\u{CB5}' {
        return false;
    } else if c >= '\u{CB5}' && c <= '\u{CB9}' {
        return true;
    } else if c < '\u{CBC}' {
        return false;
    } else if c >= '\u{CBC}' && c <= '\u{CC4}' {
        return true;
    } else if c < '\u{CC6}' {
        return false;
    } else if c >= '\u{CC6}' && c <= '\u{CC8}' {
        return true;
    } else if c < '\u{CCA}' {
        return false;
    } else if c >= '\u{CCA}' && c <= '\u{CCD}' {
        return true;
    } else if c < '\u{CD5}' {
        return false;
    } else if c >= '\u{CD5}' && c <= '\u{CD6}' {
        return true;
    } else if c < '\u{CDE}' {
        return false;
    } else if c >= '\u{CDE}' && c <= '\u{CDE}' {
        return true;
    } else if c < '\u{CE0}' {
        return false;
    } else if c >= '\u{CE0}' && c <= '\u{CE3}' {
        return true;
    } else if c < '\u{CE6}' {
        return false;
    } else if c >= '\u{CE6}' && c <= '\u{CEF}' {
        return true;
    } else if c < '\u{CF1}' {
        return false;
    } else if c >= '\u{CF1}' && c <= '\u{CF2}' {
        return true;
    } else if c < '\u{D01}' {
        return false;
    } else if c >= '\u{D01}' && c <= '\u{D03}' {
        return true;
    } else if c < '\u{D05}' {
        return false;
    } else if c >= '\u{D05}' && c <= '\u{D0C}' {
        return true;
    } else if c < '\u{D0E}' {
        return false;
    } else if c >= '\u{D0E}' && c <= '\u{D10}' {
        return true;
    } else if c < '\u{D12}' {
        return false;
    } else if c >= '\u{D12}' && c <= '\u{D3A}' {
        return true;
    } else if c < '\u{D3D}' {
        return false;
    } else if c >= '\u{D3D}' && c <= '\u{D44}' {
        return true;
    } else if c < '\u{D46}' {
        return false;
    } else if c >= '\u{D46}' && c <= '\u{D48}' {
        return true;
    } else if c < '\u{D4A}' {
        return false;
    } else if c >= '\u{D4A}' && c <= '\u{D4E}' {
        return true;
    } else if c < '\u{D54}' {
        return false;
    } else if c >= '\u{D54}' && c <= '\u{D57}' {
        return true;
    } else if c < '\u{D5F}' {
        return false;
    } else if c >= '\u{D5F}' && c <= '\u{D63}' {
        return true;
    } else if c < '\u{D66}' {
        return false;
    } else if c >= '\u{D66}' && c <= '\u{D6F}' {
        return true;
    } else if c < '\u{D7A}' {
        return false;
    } else if c >= '\u{D7A}' && c <= '\u{D7F}' {
        return true;
    } else if c < '\u{D82}' {
        return false;
    } else if c >= '\u{D82}' && c <= '\u{D83}' {
        return true;
    } else if c < '\u{D85}' {
        return false;
    } else if c >= '\u{D85}' && c <= '\u{D96}' {
        return true;
    } else if c < '\u{D9A}' {
        return false;
    } else if c >= '\u{D9A}' && c <= '\u{DB1}' {
        return true;
    } else if c < '\u{DB3}' {
        return false;
    } else if c >= '\u{DB3}' && c <= '\u{DBB}' {
        return true;
    } else if c < '\u{DBD}' {
        return false;
    } else if c >= '\u{DBD}' && c <= '\u{DBD}' {
        return true;
    } else if c < '\u{DC0}' {
        return false;
    } else if c >= '\u{DC0}' && c <= '\u{DC6}' {
        return true;
    } else if c < '\u{DCA}' {
        return false;
    } else if c >= '\u{DCA}' && c <= '\u{DCA}' {
        return true;
    } else if c < '\u{DCF}' {
        return false;
    } else if c >= '\u{DCF}' && c <= '\u{DD4}' {
        return true;
    } else if c < '\u{DD6}' {
        return false;
    } else if c >= '\u{DD6}' && c <= '\u{DD6}' {
        return true;
    } else if c < '\u{DD8}' {
        return false;
    } else if c >= '\u{DD8}' && c <= '\u{DDF}' {
        return true;
    } else if c < '\u{DE6}' {
        return false;
    } else if c >= '\u{DE6}' && c <= '\u{DEF}' {
        return true;
    } else if c < '\u{DF2}' {
        return false;
    } else if c >= '\u{DF2}' && c <= '\u{DF3}' {
        return true;
    } else if c < '\u{E01}' {
        return false;
    } else if c >= '\u{E01}' && c <= '\u{E3A}' {
        return true;
    } else if c < '\u{E40}' {
        return false;
    } else if c >= '\u{E40}' && c <= '\u{E4E}' {
        return true;
    } else if c < '\u{E50}' {
        return false;
    } else if c >= '\u{E50}' && c <= '\u{E59}' {
        return true;
    } else if c < '\u{E81}' {
        return false;
    } else if c >= '\u{E81}' && c <= '\u{E82}' {
        return true;
    } else if c < '\u{E84}' {
        return false;
    } else if c >= '\u{E84}' && c <= '\u{E84}' {
        return true;
    } else if c < '\u{E87}' {
        return false;
    } else if c >= '\u{E87}' && c <= '\u{E88}' {
        return true;
    } else if c < '\u{E8A}' {
        return false;
    } else if c >= '\u{E8A}' && c <= '\u{E8A}' {
        return true;
    } else if c < '\u{E8D}' {
        return false;
    } else if c >= '\u{E8D}' && c <= '\u{E8D}' {
        return true;
    } else if c < '\u{E94}' {
        return false;
    } else if c >= '\u{E94}' && c <= '\u{E97}' {
        return true;
    } else if c < '\u{E99}' {
        return false;
    } else if c >= '\u{E99}' && c <= '\u{E9F}' {
        return true;
    } else if c < '\u{EA1}' {
        return false;
    } else if c >= '\u{EA1}' && c <= '\u{EA3}' {
        return true;
    } else if c < '\u{EA5}' {
        return false;
    } else if c >= '\u{EA5}' && c <= '\u{EA5}' {
        return true;
    } else if c < '\u{EA7}' {
        return false;
    } else if c >= '\u{EA7}' && c <= '\u{EA7}' {
        return true;
    } else if c < '\u{EAA}' {
        return false;
    } else if c >= '\u{EAA}' && c <= '\u{EAB}' {
        return true;
    } else if c < '\u{EAD}' {
        return false;
    } else if c >= '\u{EAD}' && c <= '\u{EB9}' {
        return true;
    } else if c < '\u{EBB}' {
        return false;
    } else if c >= '\u{EBB}' && c <= '\u{EBD}' {
        return true;
    } else if c < '\u{EC0}' {
        return false;
    } else if c >= '\u{EC0}' && c <= '\u{EC4}' {
        return true;
    } else if c < '\u{EC6}' {
        return false;
    } else if c >= '\u{EC6}' && c <= '\u{EC6}' {
        return true;
    } else if c < '\u{EC8}' {
        return false;
    } else if c >= '\u{EC8}' && c <= '\u{ECD}' {
        return true;
    } else if c < '\u{ED0}' {
        return false;
    } else if c >= '\u{ED0}' && c <= '\u{ED9}' {
        return true;
    } else if c < '\u{EDC}' {
        return false;
    } else if c >= '\u{EDC}' && c <= '\u{EDF}' {
        return true;
    } else if c < '\u{F00}' {
        return false;
    } else if c >= '\u{F00}' && c <= '\u{F00}' {
        return true;
    } else if c < '\u{F18}' {
        return false;
    } else if c >= '\u{F18}' && c <= '\u{F19}' {
        return true;
    } else if c < '\u{F20}' {
        return false;
    } else if c >= '\u{F20}' && c <= '\u{F29}' {
        return true;
    } else if c < '\u{F35}' {
        return false;
    } else if c >= '\u{F35}' && c <= '\u{F35}' {
        return true;
    } else if c < '\u{F37}' {
        return false;
    } else if c >= '\u{F37}' && c <= '\u{F37}' {
        return true;
    } else if c < '\u{F39}' {
        return false;
    } else if c >= '\u{F39}' && c <= '\u{F39}' {
        return true;
    } else if c < '\u{F3E}' {
        return false;
    } else if c >= '\u{F3E}' && c <= '\u{F47}' {
        return true;
    } else if c < '\u{F49}' {
        return false;
    } else if c >= '\u{F49}' && c <= '\u{F6C}' {
        return true;
    } else if c < '\u{F71}' {
        return false;
    } else if c >= '\u{F71}' && c <= '\u{F84}' {
        return true;
    } else if c < '\u{F86}' {
        return false;
    } else if c >= '\u{F86}' && c <= '\u{F97}' {
        return true;
    } else if c < '\u{F99}' {
        return false;
    } else if c >= '\u{F99}' && c <= '\u{FBC}' {
        return true;
    } else if c < '\u{FC6}' {
        return false;
    } else if c >= '\u{FC6}' && c <= '\u{FC6}' {
        return true;
    } else if c < '\u{1000}' {
        return false;
    } else if c >= '\u{1000}' && c <= '\u{1049}' {
        return true;
    } else if c < '\u{1050}' {
        return false;
    } else if c >= '\u{1050}' && c <= '\u{109D}' {
        return true;
    } else if c < '\u{10A0}' {
        return false;
    } else if c >= '\u{10A0}' && c <= '\u{10C5}' {
        return true;
    } else if c < '\u{10C7}' {
        return false;
    } else if c >= '\u{10C7}' && c <= '\u{10C7}' {
        return true;
    } else if c < '\u{10CD}' {
        return false;
    } else if c >= '\u{10CD}' && c <= '\u{10CD}' {
        return true;
    } else if c < '\u{10D0}' {
        return false;
    } else if c >= '\u{10D0}' && c <= '\u{10FA}' {
        return true;
    } else if c < '\u{10FC}' {
        return false;
    } else if c >= '\u{10FC}' && c <= '\u{1248}' {
        return true;
    } else if c < '\u{124A}' {
        return false;
    } else if c >= '\u{124A}' && c <= '\u{124D}' {
        return true;
    } else if c < '\u{1250}' {
        return false;
    } else if c >= '\u{1250}' && c <= '\u{1256}' {
        return true;
    } else if c < '\u{1258}' {
        return false;
    } else if c >= '\u{1258}' && c <= '\u{1258}' {
        return true;
    } else if c < '\u{125A}' {
        return false;
    } else if c >= '\u{125A}' && c <= '\u{125D}' {
        return true;
    } else if c < '\u{1260}' {
        return false;
    } else if c >= '\u{1260}' && c <= '\u{1288}' {
        return true;
    } else if c < '\u{128A}' {
        return false;
    } else if c >= '\u{128A}' && c <= '\u{128D}' {
        return true;
    } else if c < '\u{1290}' {
        return false;
    } else if c >= '\u{1290}' && c <= '\u{12B0}' {
        return true;
    } else if c < '\u{12B2}' {
        return false;
    } else if c >= '\u{12B2}' && c <= '\u{12B5}' {
        return true;
    } else if c < '\u{12B8}' {
        return false;
    } else if c >= '\u{12B8}' && c <= '\u{12BE}' {
        return true;
    } else if c < '\u{12C0}' {
        return false;
    } else if c >= '\u{12C0}' && c <= '\u{12C0}' {
        return true;
    } else if c < '\u{12C2}' {
        return false;
    } else if c >= '\u{12C2}' && c <= '\u{12C5}' {
        return true;
    } else if c < '\u{12C8}' {
        return false;
    } else if c >= '\u{12C8}' && c <= '\u{12D6}' {
        return true;
    } else if c < '\u{12D8}' {
        return false;
    } else if c >= '\u{12D8}' && c <= '\u{1310}' {
        return true;
    } else if c < '\u{1312}' {
        return false;
    } else if c >= '\u{1312}' && c <= '\u{1315}' {
        return true;
    } else if c < '\u{1318}' {
        return false;
    } else if c >= '\u{1318}' && c <= '\u{135A}' {
        return true;
    } else if c < '\u{135D}' {
        return false;
    } else if c >= '\u{135D}' && c <= '\u{135F}' {
        return true;
    } else if c < '\u{1369}' {
        return false;
    } else if c >= '\u{1369}' && c <= '\u{1371}' {
        return true;
    } else if c < '\u{1380}' {
        return false;
    } else if c >= '\u{1380}' && c <= '\u{138F}' {
        return true;
    } else if c < '\u{13A0}' {
        return false;
    } else if c >= '\u{13A0}' && c <= '\u{13F5}' {
        return true;
    } else if c < '\u{13F8}' {
        return false;
    } else if c >= '\u{13F8}' && c <= '\u{13FD}' {
        return true;
    } else if c < '\u{1401}' {
        return false;
    } else if c >= '\u{1401}' && c <= '\u{166C}' {
        return true;
    } else if c < '\u{166F}' {
        return false;
    } else if c >= '\u{166F}' && c <= '\u{167F}' {
        return true;
    } else if c < '\u{1681}' {
        return false;
    } else if c >= '\u{1681}' && c <= '\u{169A}' {
        return true;
    } else if c < '\u{16A0}' {
        return false;
    } else if c >= '\u{16A0}' && c <= '\u{16EA}' {
        return true;
    } else if c < '\u{16EE}' {
        return false;
    } else if c >= '\u{16EE}' && c <= '\u{16F8}' {
        return true;
    } else if c < '\u{1700}' {
        return false;
    } else if c >= '\u{1700}' && c <= '\u{170C}' {
        return true;
    } else if c < '\u{170E}' {
        return false;
    } else if c >= '\u{170E}' && c <= '\u{1714}' {
        return true;
    } else if c < '\u{1720}' {
        return false;
    } else if c >= '\u{1720}' && c <= '\u{1734}' {
        return true;
    } else if c < '\u{1740}' {
        return false;
    } else if c >= '\u{1740}' && c <= '\u{1753}' {
        return true;
    } else if c < '\u{1760}' {
        return false;
    } else if c >= '\u{1760}' && c <= '\u{176C}' {
        return true;
    } else if c < '\u{176E}' {
        return false;
    } else if c >= '\u{176E}' && c <= '\u{1770}' {
        return true;
    } else if c < '\u{1772}' {
        return false;
    } else if c >= '\u{1772}' && c <= '\u{1773}' {
        return true;
    } else if c < '\u{1780}' {
        return false;
    } else if c >= '\u{1780}' && c <= '\u{17D3}' {
        return true;
    } else if c < '\u{17D7}' {
        return false;
    } else if c >= '\u{17D7}' && c <= '\u{17D7}' {
        return true;
    } else if c < '\u{17DC}' {
        return false;
    } else if c >= '\u{17DC}' && c <= '\u{17DD}' {
        return true;
    } else if c < '\u{17E0}' {
        return false;
    } else if c >= '\u{17E0}' && c <= '\u{17E9}' {
        return true;
    } else if c < '\u{180B}' {
        return false;
    } else if c >= '\u{180B}' && c <= '\u{180D}' {
        return true;
    } else if c < '\u{1810}' {
        return false;
    } else if c >= '\u{1810}' && c <= '\u{1819}' {
        return true;
    } else if c < '\u{1820}' {
        return false;
    } else if c >= '\u{1820}' && c <= '\u{1877}' {
        return true;
    } else if c < '\u{1880}' {
        return false;
    } else if c >= '\u{1880}' && c <= '\u{18AA}' {
        return true;
    } else if c < '\u{18B0}' {
        return false;
    } else if c >= '\u{18B0}' && c <= '\u{18F5}' {
        return true;
    } else if c < '\u{1900}' {
        return false;
    } else if c >= '\u{1900}' && c <= '\u{191E}' {
        return true;
    } else if c < '\u{1920}' {
        return false;
    } else if c >= '\u{1920}' && c <= '\u{192B}' {
        return true;
    } else if c < '\u{1930}' {
        return false;
    } else if c >= '\u{1930}' && c <= '\u{193B}' {
        return true;
    } else if c < '\u{1946}' {
        return false;
    } else if c >= '\u{1946}' && c <= '\u{196D}' {
        return true;
    } else if c < '\u{1970}' {
        return false;
    } else if c >= '\u{1970}' && c <= '\u{1974}' {
        return true;
    } else if c < '\u{1980}' {
        return false;
    } else if c >= '\u{1980}' && c <= '\u{19AB}' {
        return true;
    } else if c < '\u{19B0}' {
        return false;
    } else if c >= '\u{19B0}' && c <= '\u{19C9}' {
        return true;
    } else if c < '\u{19D0}' {
        return false;
    } else if c >= '\u{19D0}' && c <= '\u{19DA}' {
        return true;
    } else if c < '\u{1A00}' {
        return false;
    } else if c >= '\u{1A00}' && c <= '\u{1A1B}' {
        return true;
    } else if c < '\u{1A20}' {
        return false;
    } else if c >= '\u{1A20}' && c <= '\u{1A5E}' {
        return true;
    } else if c < '\u{1A60}' {
        return false;
    } else if c >= '\u{1A60}' && c <= '\u{1A7C}' {
        return true;
    } else if c < '\u{1A7F}' {
        return false;
    } else if c >= '\u{1A7F}' && c <= '\u{1A89}' {
        return true;
    } else if c < '\u{1A90}' {
        return false;
    } else if c >= '\u{1A90}' && c <= '\u{1A99}' {
        return true;
    } else if c < '\u{1AA7}' {
        return false;
    } else if c >= '\u{1AA7}' && c <= '\u{1AA7}' {
        return true;
    } else if c < '\u{1AB0}' {
        return false;
    } else if c >= '\u{1AB0}' && c <= '\u{1ABD}' {
        return true;
    } else if c < '\u{1B00}' {
        return false;
    } else if c >= '\u{1B00}' && c <= '\u{1B4B}' {
        return true;
    } else if c < '\u{1B50}' {
        return false;
    } else if c >= '\u{1B50}' && c <= '\u{1B59}' {
        return true;
    } else if c < '\u{1B6B}' {
        return false;
    } else if c >= '\u{1B6B}' && c <= '\u{1B73}' {
        return true;
    } else if c < '\u{1B80}' {
        return false;
    } else if c >= '\u{1B80}' && c <= '\u{1BF3}' {
        return true;
    } else if c < '\u{1C00}' {
        return false;
    } else if c >= '\u{1C00}' && c <= '\u{1C37}' {
        return true;
    } else if c < '\u{1C40}' {
        return false;
    } else if c >= '\u{1C40}' && c <= '\u{1C49}' {
        return true;
    } else if c < '\u{1C4D}' {
        return false;
    } else if c >= '\u{1C4D}' && c <= '\u{1C7D}' {
        return true;
    } else if c < '\u{1C80}' {
        return false;
    } else if c >= '\u{1C80}' && c <= '\u{1C88}' {
        return true;
    } else if c < '\u{1CD0}' {
        return false;
    } else if c >= '\u{1CD0}' && c <= '\u{1CD2}' {
        return true;
    } else if c < '\u{1CD4}' {
        return false;
    } else if c >= '\u{1CD4}' && c <= '\u{1CF6}' {
        return true;
    } else if c < '\u{1CF8}' {
        return false;
    } else if c >= '\u{1CF8}' && c <= '\u{1CF9}' {
        return true;
    } else if c < '\u{1D00}' {
        return false;
    } else if c >= '\u{1D00}' && c <= '\u{1DF5}' {
        return true;
    } else if c < '\u{1DFB}' {
        return false;
    } else if c >= '\u{1DFB}' && c <= '\u{1F15}' {
        return true;
    } else if c < '\u{1F18}' {
        return false;
    } else if c >= '\u{1F18}' && c <= '\u{1F1D}' {
        return true;
    } else if c < '\u{1F20}' {
        return false;
    } else if c >= '\u{1F20}' && c <= '\u{1F45}' {
        return true;
    } else if c < '\u{1F48}' {
        return false;
    } else if c >= '\u{1F48}' && c <= '\u{1F4D}' {
        return true;
    } else if c < '\u{1F50}' {
        return false;
    } else if c >= '\u{1F50}' && c <= '\u{1F57}' {
        return true;
    } else if c < '\u{1F59}' {
        return false;
    } else if c >= '\u{1F59}' && c <= '\u{1F59}' {
        return true;
    } else if c < '\u{1F5B}' {
        return false;
    } else if c >= '\u{1F5B}' && c <= '\u{1F5B}' {
        return true;
    } else if c < '\u{1F5D}' {
        return false;
    } else if c >= '\u{1F5D}' && c <= '\u{1F5D}' {
        return true;
    } else if c < '\u{1F5F}' {
        return false;
    } else if c >= '\u{1F5F}' && c <= '\u{1F7D}' {
        return true;
    } else if c < '\u{1F80}' {
        return false;
    } else if c >= '\u{1F80}' && c <= '\u{1FB4}' {
        return true;
    } else if c < '\u{1FB6}' {
        return false;
    } else if c >= '\u{1FB6}' && c <= '\u{1FBC}' {
        return true;
    } else if c < '\u{1FBE}' {
        return false;
    } else if c >= '\u{1FBE}' && c <= '\u{1FBE}' {
        return true;
    } else if c < '\u{1FC2}' {
        return false;
    } else if c >= '\u{1FC2}' && c <= '\u{1FC4}' {
        return true;
    } else if c < '\u{1FC6}' {
        return false;
    } else if c >= '\u{1FC6}' && c <= '\u{1FCC}' {
        return true;
    } else if c < '\u{1FD0}' {
        return false;
    } else if c >= '\u{1FD0}' && c <= '\u{1FD3}' {
        return true;
    } else if c < '\u{1FD6}' {
        return false;
    } else if c >= '\u{1FD6}' && c <= '\u{1FDB}' {
        return true;
    } else if c < '\u{1FE0}' {
        return false;
    } else if c >= '\u{1FE0}' && c <= '\u{1FEC}' {
        return true;
    } else if c < '\u{1FF2}' {
        return false;
    } else if c >= '\u{1FF2}' && c <= '\u{1FF4}' {
        return true;
    } else if c < '\u{1FF6}' {
        return false;
    } else if c >= '\u{1FF6}' && c <= '\u{1FFC}' {
        return true;
    } else if c < '\u{203F}' {
        return false;
    } else if c >= '\u{203F}' && c <= '\u{2040}' {
        return true;
    } else if c < '\u{2054}' {
        return false;
    } else if c >= '\u{2054}' && c <= '\u{2054}' {
        return true;
    } else if c < '\u{2071}' {
        return false;
    } else if c >= '\u{2071}' && c <= '\u{2071}' {
        return true;
    } else if c < '\u{207F}' {
        return false;
    } else if c >= '\u{207F}' && c <= '\u{207F}' {
        return true;
    } else if c < '\u{2090}' {
        return false;
    } else if c >= '\u{2090}' && c <= '\u{209C}' {
        return true;
    } else if c < '\u{20D0}' {
        return false;
    } else if c >= '\u{20D0}' && c <= '\u{20DC}' {
        return true;
    } else if c < '\u{20E1}' {
        return false;
    } else if c >= '\u{20E1}' && c <= '\u{20E1}' {
        return true;
    } else if c < '\u{20E5}' {
        return false;
    } else if c >= '\u{20E5}' && c <= '\u{20F0}' {
        return true;
    } else if c < '\u{2102}' {
        return false;
    } else if c >= '\u{2102}' && c <= '\u{2102}' {
        return true;
    } else if c < '\u{2107}' {
        return false;
    } else if c >= '\u{2107}' && c <= '\u{2107}' {
        return true;
    } else if c < '\u{210A}' {
        return false;
    } else if c >= '\u{210A}' && c <= '\u{2113}' {
        return true;
    } else if c < '\u{2115}' {
        return false;
    } else if c >= '\u{2115}' && c <= '\u{2115}' {
        return true;
    } else if c < '\u{2118}' {
        return false;
    } else if c >= '\u{2118}' && c <= '\u{211D}' {
        return true;
    } else if c < '\u{2124}' {
        return false;
    } else if c >= '\u{2124}' && c <= '\u{2124}' {
        return true;
    } else if c < '\u{2126}' {
        return false;
    } else if c >= '\u{2126}' && c <= '\u{2126}' {
        return true;
    } else if c < '\u{2128}' {
        return false;
    } else if c >= '\u{2128}' && c <= '\u{2128}' {
        return true;
    } else if c < '\u{212A}' {
        return false;
    } else if c >= '\u{212A}' && c <= '\u{2139}' {
        return true;
    } else if c < '\u{213C}' {
        return false;
    } else if c >= '\u{213C}' && c <= '\u{213F}' {
        return true;
    } else if c < '\u{2145}' {
        return false;
    } else if c >= '\u{2145}' && c <= '\u{2149}' {
        return true;
    } else if c < '\u{214E}' {
        return false;
    } else if c >= '\u{214E}' && c <= '\u{214E}' {
        return true;
    } else if c < '\u{2160}' {
        return false;
    } else if c >= '\u{2160}' && c <= '\u{2188}' {
        return true;
    } else if c < '\u{2C00}' {
        return false;
    } else if c >= '\u{2C00}' && c <= '\u{2C2E}' {
        return true;
    } else if c < '\u{2C30}' {
        return false;
    } else if c >= '\u{2C30}' && c <= '\u{2C5E}' {
        return true;
    } else if c < '\u{2C60}' {
        return false;
    } else if c >= '\u{2C60}' && c <= '\u{2CE4}' {
        return true;
    } else if c < '\u{2CEB}' {
        return false;
    } else if c >= '\u{2CEB}' && c <= '\u{2CF3}' {
        return true;
    } else if c < '\u{2D00}' {
        return false;
    } else if c >= '\u{2D00}' && c <= '\u{2D25}' {
        return true;
    } else if c < '\u{2D27}' {
        return false;
    } else if c >= '\u{2D27}' && c <= '\u{2D27}' {
        return true;
    } else if c < '\u{2D2D}' {
        return false;
    } else if c >= '\u{2D2D}' && c <= '\u{2D2D}' {
        return true;
    } else if c < '\u{2D30}' {
        return false;
    } else if c >= '\u{2D30}' && c <= '\u{2D67}' {
        return true;
    } else if c < '\u{2D6F}' {
        return false;
    } else if c >= '\u{2D6F}' && c <= '\u{2D6F}' {
        return true;
    } else if c < '\u{2D7F}' {
        return false;
    } else if c >= '\u{2D7F}' && c <= '\u{2D96}' {
        return true;
    } else if c < '\u{2DA0}' {
        return false;
    } else if c >= '\u{2DA0}' && c <= '\u{2DA6}' {
        return true;
    } else if c < '\u{2DA8}' {
        return false;
    } else if c >= '\u{2DA8}' && c <= '\u{2DAE}' {
        return true;
    } else if c < '\u{2DB0}' {
        return false;
    } else if c >= '\u{2DB0}' && c <= '\u{2DB6}' {
        return true;
    } else if c < '\u{2DB8}' {
        return false;
    } else if c >= '\u{2DB8}' && c <= '\u{2DBE}' {
        return true;
    } else if c < '\u{2DC0}' {
        return false;
    } else if c >= '\u{2DC0}' && c <= '\u{2DC6}' {
        return true;
    } else if c < '\u{2DC8}' {
        return false;
    } else if c >= '\u{2DC8}' && c <= '\u{2DCE}' {
        return true;
    } else if c < '\u{2DD0}' {
        return false;
    } else if c >= '\u{2DD0}' && c <= '\u{2DD6}' {
        return true;
    } else if c < '\u{2DD8}' {
        return false;
    } else if c >= '\u{2DD8}' && c <= '\u{2DDE}' {
        return true;
    } else if c < '\u{2DE0}' {
        return false;
    } else if c >= '\u{2DE0}' && c <= '\u{2DFF}' {
        return true;
    } else if c < '\u{3005}' {
        return false;
    } else if c >= '\u{3005}' && c <= '\u{3007}' {
        return true;
    } else if c < '\u{3021}' {
        return false;
    } else if c >= '\u{3021}' && c <= '\u{302F}' {
        return true;
    } else if c < '\u{3031}' {
        return false;
    } else if c >= '\u{3031}' && c <= '\u{3035}' {
        return true;
    } else if c < '\u{3038}' {
        return false;
    } else if c >= '\u{3038}' && c <= '\u{303C}' {
        return true;
    } else if c < '\u{3041}' {
        return false;
    } else if c >= '\u{3041}' && c <= '\u{3096}' {
        return true;
    } else if c < '\u{3099}' {
        return false;
    } else if c >= '\u{3099}' && c <= '\u{309F}' {
        return true;
    } else if c < '\u{30A1}' {
        return false;
    } else if c >= '\u{30A1}' && c <= '\u{30FA}' {
        return true;
    } else if c < '\u{30FC}' {
        return false;
    } else if c >= '\u{30FC}' && c <= '\u{30FF}' {
        return true;
    } else if c < '\u{3105}' {
        return false;
    } else if c >= '\u{3105}' && c <= '\u{312D}' {
        return true;
    } else if c < '\u{3131}' {
        return false;
    } else if c >= '\u{3131}' && c <= '\u{318E}' {
        return true;
    } else if c < '\u{31A0}' {
        return false;
    } else if c >= '\u{31A0}' && c <= '\u{31BA}' {
        return true;
    } else if c < '\u{31F0}' {
        return false;
    } else if c >= '\u{31F0}' && c <= '\u{31FF}' {
        return true;
    } else if c < '\u{3400}' {
        return false;
    } else if c >= '\u{3400}' && c <= '\u{4DB5}' {
        return true;
    } else if c < '\u{4E00}' {
        return false;
    } else if c >= '\u{4E00}' && c <= '\u{9FD5}' {
        return true;
    } else if c < '\u{A000}' {
        return false;
    } else if c >= '\u{A000}' && c <= '\u{A48C}' {
        return true;
    } else if c < '\u{A4D0}' {
        return false;
    } else if c >= '\u{A4D0}' && c <= '\u{A4FD}' {
        return true;
    } else if c < '\u{A500}' {
        return false;
    } else if c >= '\u{A500}' && c <= '\u{A60C}' {
        return true;
    } else if c < '\u{A610}' {
        return false;
    } else if c >= '\u{A610}' && c <= '\u{A62B}' {
        return true;
    } else if c < '\u{A640}' {
        return false;
    } else if c >= '\u{A640}' && c <= '\u{A66F}' {
        return true;
    } else if c < '\u{A674}' {
        return false;
    } else if c >= '\u{A674}' && c <= '\u{A67D}' {
        return true;
    } else if c < '\u{A67F}' {
        return false;
    } else if c >= '\u{A67F}' && c <= '\u{A6F1}' {
        return true;
    } else if c < '\u{A717}' {
        return false;
    } else if c >= '\u{A717}' && c <= '\u{A71F}' {
        return true;
    } else if c < '\u{A722}' {
        return false;
    } else if c >= '\u{A722}' && c <= '\u{A788}' {
        return true;
    } else if c < '\u{A78B}' {
        return false;
    } else if c >= '\u{A78B}' && c <= '\u{A7AE}' {
        return true;
    } else if c < '\u{A7B0}' {
        return false;
    } else if c >= '\u{A7B0}' && c <= '\u{A7B7}' {
        return true;
    } else if c < '\u{A7F7}' {
        return false;
    } else if c >= '\u{A7F7}' && c <= '\u{A827}' {
        return true;
    } else if c < '\u{A840}' {
        return false;
    } else if c >= '\u{A840}' && c <= '\u{A873}' {
        return true;
    } else if c < '\u{A880}' {
        return false;
    } else if c >= '\u{A880}' && c <= '\u{A8C5}' {
        return true;
    } else if c < '\u{A8D0}' {
        return false;
    } else if c >= '\u{A8D0}' && c <= '\u{A8D9}' {
        return true;
    } else if c < '\u{A8E0}' {
        return false;
    } else if c >= '\u{A8E0}' && c <= '\u{A8F7}' {
        return true;
    } else if c < '\u{A8FB}' {
        return false;
    } else if c >= '\u{A8FB}' && c <= '\u{A8FB}' {
        return true;
    } else if c < '\u{A8FD}' {
        return false;
    } else if c >= '\u{A8FD}' && c <= '\u{A8FD}' {
        return true;
    } else if c < '\u{A900}' {
        return false;
    } else if c >= '\u{A900}' && c <= '\u{A92D}' {
        return true;
    } else if c < '\u{A930}' {
        return false;
    } else if c >= '\u{A930}' && c <= '\u{A953}' {
        return true;
    } else if c < '\u{A960}' {
        return false;
    } else if c >= '\u{A960}' && c <= '\u{A97C}' {
        return true;
    } else if c < '\u{A980}' {
        return false;
    } else if c >= '\u{A980}' && c <= '\u{A9C0}' {
        return true;
    } else if c < '\u{A9CF}' {
        return false;
    } else if c >= '\u{A9CF}' && c <= '\u{A9D9}' {
        return true;
    } else if c < '\u{A9E0}' {
        return false;
    } else if c >= '\u{A9E0}' && c <= '\u{A9FE}' {
        return true;
    } else if c < '\u{AA00}' {
        return false;
    } else if c >= '\u{AA00}' && c <= '\u{AA36}' {
        return true;
    } else if c < '\u{AA40}' {
        return false;
    } else if c >= '\u{AA40}' && c <= '\u{AA4D}' {
        return true;
    } else if c < '\u{AA50}' {
        return false;
    } else if c >= '\u{AA50}' && c <= '\u{AA59}' {
        return true;
    } else if c < '\u{AA60}' {
        return false;
    } else if c >= '\u{AA60}' && c <= '\u{AA76}' {
        return true;
    } else if c < '\u{AA7A}' {
        return false;
    } else if c >= '\u{AA7A}' && c <= '\u{AAC2}' {
        return true;
    } else if c < '\u{AADB}' {
        return false;
    } else if c >= '\u{AADB}' && c <= '\u{AADD}' {
        return true;
    } else if c < '\u{AAE0}' {
        return false;
    } else if c >= '\u{AAE0}' && c <= '\u{AAEF}' {
        return true;
    } else if c < '\u{AAF2}' {
        return false;
    } else if c >= '\u{AAF2}' && c <= '\u{AAF6}' {
        return true;
    } else if c < '\u{AB01}' {
        return false;
    } else if c >= '\u{AB01}' && c <= '\u{AB06}' {
        return true;
    } else if c < '\u{AB09}' {
        return false;
    } else if c >= '\u{AB09}' && c <= '\u{AB0E}' {
        return true;
    } else if c < '\u{AB11}' {
        return false;
    } else if c >= '\u{AB11}' && c <= '\u{AB16}' {
        return true;
    } else if c < '\u{AB20}' {
        return false;
    } else if c >= '\u{AB20}' && c <= '\u{AB26}' {
        return true;
    } else if c < '\u{AB28}' {
        return false;
    } else if c >= '\u{AB28}' && c <= '\u{AB2E}' {
        return true;
    } else if c < '\u{AB30}' {
        return false;
    } else if c >= '\u{AB30}' && c <= '\u{AB5A}' {
        return true;
    } else if c < '\u{AB5C}' {
        return false;
    } else if c >= '\u{AB5C}' && c <= '\u{AB65}' {
        return true;
    } else if c < '\u{AB70}' {
        return false;
    } else if c >= '\u{AB70}' && c <= '\u{ABEA}' {
        return true;
    } else if c < '\u{ABEC}' {
        return false;
    } else if c >= '\u{ABEC}' && c <= '\u{ABED}' {
        return true;
    } else if c < '\u{ABF0}' {
        return false;
    } else if c >= '\u{ABF0}' && c <= '\u{ABF9}' {
        return true;
    } else if c < '\u{AC00}' {
        return false;
    } else if c >= '\u{AC00}' && c <= '\u{D7A3}' {
        return true;
    } else if c < '\u{D7B0}' {
        return false;
    } else if c >= '\u{D7B0}' && c <= '\u{D7C6}' {
        return true;
    } else if c < '\u{D7CB}' {
        return false;
    } else if c >= '\u{D7CB}' && c <= '\u{D7FB}' {
        return true;
    } else if c < '\u{F900}' {
        return false;
    } else if c >= '\u{F900}' && c <= '\u{FA6D}' {
        return true;
    } else if c < '\u{FA70}' {
        return false;
    } else if c >= '\u{FA70}' && c <= '\u{FAD9}' {
        return true;
    } else if c < '\u{FB00}' {
        return false;
    } else if c >= '\u{FB00}' && c <= '\u{FB06}' {
        return true;
    } else if c < '\u{FB13}' {
        return false;
    } else if c >= '\u{FB13}' && c <= '\u{FB17}' {
        return true;
    } else if c < '\u{FB1D}' {
        return false;
    } else if c >= '\u{FB1D}' && c <= '\u{FB28}' {
        return true;
    } else if c < '\u{FB2A}' {
        return false;
    } else if c >= '\u{FB2A}' && c <= '\u{FB36}' {
        return true;
    } else if c < '\u{FB38}' {
        return false;
    } else if c >= '\u{FB38}' && c <= '\u{FB3C}' {
        return true;
    } else if c < '\u{FB3E}' {
        return false;
    } else if c >= '\u{FB3E}' && c <= '\u{FB3E}' {
        return true;
    } else if c < '\u{FB40}' {
        return false;
    } else if c >= '\u{FB40}' && c <= '\u{FB41}' {
        return true;
    } else if c < '\u{FB43}' {
        return false;
    } else if c >= '\u{FB43}' && c <= '\u{FB44}' {
        return true;
    } else if c < '\u{FB46}' {
        return false;
    } else if c >= '\u{FB46}' && c <= '\u{FBB1}' {
        return true;
    } else if c < '\u{FBD3}' {
        return false;
    } else if c >= '\u{FBD3}' && c <= '\u{FD3D}' {
        return true;
    } else if c < '\u{FD50}' {
        return false;
    } else if c >= '\u{FD50}' && c <= '\u{FD8F}' {
        return true;
    } else if c < '\u{FD92}' {
        return false;
    } else if c >= '\u{FD92}' && c <= '\u{FDC7}' {
        return true;
    } else if c < '\u{FDF0}' {
        return false;
    } else if c >= '\u{FDF0}' && c <= '\u{FDFB}' {
        return true;
    } else if c < '\u{FE00}' {
        return false;
    } else if c >= '\u{FE00}' && c <= '\u{FE0F}' {
        return true;
    } else if c < '\u{FE20}' {
        return false;
    } else if c >= '\u{FE20}' && c <= '\u{FE2F}' {
        return true;
    } else if c < '\u{FE33}' {
        return false;
    } else if c >= '\u{FE33}' && c <= '\u{FE34}' {
        return true;
    } else if c < '\u{FE4D}' {
        return false;
    } else if c >= '\u{FE4D}' && c <= '\u{FE4F}' {
        return true;
    } else if c < '\u{FE70}' {
        return false;
    } else if c >= '\u{FE70}' && c <= '\u{FE74}' {
        return true;
    } else if c < '\u{FE76}' {
        return false;
    } else if c >= '\u{FE76}' && c <= '\u{FEFC}' {
        return true;
    } else if c < '\u{FF10}' {
        return false;
    } else if c >= '\u{FF10}' && c <= '\u{FF19}' {
        return true;
    } else if c < '\u{FF21}' {
        return false;
    } else if c >= '\u{FF21}' && c <= '\u{FF3A}' {
        return true;
    } else if c < '\u{FF3F}' {
        return false;
    } else if c >= '\u{FF3F}' && c <= '\u{FF3F}' {
        return true;
    } else if c < '\u{FF41}' {
        return false;
    } else if c >= '\u{FF41}' && c <= '\u{FF5A}' {
        return true;
    } else if c < '\u{FF66}' {
        return false;
    } else if c >= '\u{FF66}' && c <= '\u{FFBE}' {
        return true;
    } else if c < '\u{FFC2}' {
        return false;
    } else if c >= '\u{FFC2}' && c <= '\u{FFC7}' {
        return true;
    } else if c < '\u{FFCA}' {
        return false;
    } else if c >= '\u{FFCA}' && c <= '\u{FFCF}' {
        return true;
    } else if c < '\u{FFD2}' {
        return false;
    } else if c >= '\u{FFD2}' && c <= '\u{FFD7}' {
        return true;
    } else if c < '\u{FFDA}' {
        return false;
    } else if c >= '\u{FFDA}' && c <= '\u{FFDC}' {
        return true;
    } else if c < '\u{10000}' {
        return false;
    } else if c >= '\u{10000}' && c <= '\u{1000B}' {
        return true;
    } else if c < '\u{1000D}' {
        return false;
    } else if c >= '\u{1000D}' && c <= '\u{10026}' {
        return true;
    } else if c < '\u{10028}' {
        return false;
    } else if c >= '\u{10028}' && c <= '\u{1003A}' {
        return true;
    } else if c < '\u{1003C}' {
        return false;
    } else if c >= '\u{1003C}' && c <= '\u{1003D}' {
        return true;
    } else if c < '\u{1003F}' {
        return false;
    } else if c >= '\u{1003F}' && c <= '\u{1004D}' {
        return true;
    } else if c < '\u{10050}' {
        return false;
    } else if c >= '\u{10050}' && c <= '\u{1005D}' {
        return true;
    } else if c < '\u{10080}' {
        return false;
    } else if c >= '\u{10080}' && c <= '\u{100FA}' {
        return true;
    } else if c < '\u{10140}' {
        return false;
    } else if c >= '\u{10140}' && c <= '\u{10174}' {
        return true;
    } else if c < '\u{101FD}' {
        return false;
    } else if c >= '\u{101FD}' && c <= '\u{101FD}' {
        return true;
    } else if c < '\u{10280}' {
        return false;
    } else if c >= '\u{10280}' && c <= '\u{1029C}' {
        return true;
    } else if c < '\u{102A0}' {
        return false;
    } else if c >= '\u{102A0}' && c <= '\u{102D0}' {
        return true;
    } else if c < '\u{102E0}' {
        return false;
    } else if c >= '\u{102E0}' && c <= '\u{102E0}' {
        return true;
    } else if c < '\u{10300}' {
        return false;
    } else if c >= '\u{10300}' && c <= '\u{1031F}' {
        return true;
    } else if c < '\u{10330}' {
        return false;
    } else if c >= '\u{10330}' && c <= '\u{1034A}' {
        return true;
    } else if c < '\u{10350}' {
        return false;
    } else if c >= '\u{10350}' && c <= '\u{1037A}' {
        return true;
    } else if c < '\u{10380}' {
        return false;
    } else if c >= '\u{10380}' && c <= '\u{1039D}' {
        return true;
    } else if c < '\u{103A0}' {
        return false;
    } else if c >= '\u{103A0}' && c <= '\u{103C3}' {
        return true;
    } else if c < '\u{103C8}' {
        return false;
    } else if c >= '\u{103C8}' && c <= '\u{103CF}' {
        return true;
    } else if c < '\u{103D1}' {
        return false;
    } else if c >= '\u{103D1}' && c <= '\u{103D5}' {
        return true;
    } else if c < '\u{10400}' {
        return false;
    } else if c >= '\u{10400}' && c <= '\u{1049D}' {
        return true;
    } else if c < '\u{104A0}' {
        return false;
    } else if c >= '\u{104A0}' && c <= '\u{104A9}' {
        return true;
    } else if c < '\u{104B0}' {
        return false;
    } else if c >= '\u{104B0}' && c <= '\u{104D3}' {
        return true;
    } else if c < '\u{104D8}' {
        return false;
    } else if c >= '\u{104D8}' && c <= '\u{104FB}' {
        return true;
    } else if c < '\u{10500}' {
        return false;
    } else if c >= '\u{10500}' && c <= '\u{10527}' {
        return true;
    } else if c < '\u{10530}' {
        return false;
    } else if c >= '\u{10530}' && c <= '\u{10563}' {
        return true;
    } else if c < '\u{10600}' {
        return false;
    } else if c >= '\u{10600}' && c <= '\u{10736}' {
        return true;
    } else if c < '\u{10740}' {
        return false;
    } else if c >= '\u{10740}' && c <= '\u{10755}' {
        return true;
    } else if c < '\u{10760}' {
        return false;
    } else if c >= '\u{10760}' && c <= '\u{10767}' {
        return true;
    } else if c < '\u{10800}' {
        return false;
    } else if c >= '\u{10800}' && c <= '\u{10805}' {
        return true;
    } else if c < '\u{10808}' {
        return false;
    } else if c >= '\u{10808}' && c <= '\u{10808}' {
        return true;
    } else if c < '\u{1080A}' {
        return false;
    } else if c >= '\u{1080A}' && c <= '\u{10835}' {
        return true;
    } else if c < '\u{10837}' {
        return false;
    } else if c >= '\u{10837}' && c <= '\u{10838}' {
        return true;
    } else if c < '\u{1083C}' {
        return false;
    } else if c >= '\u{1083C}' && c <= '\u{1083C}' {
        return true;
    } else if c < '\u{1083F}' {
        return false;
    } else if c >= '\u{1083F}' && c <= '\u{10855}' {
        return true;
    } else if c < '\u{10860}' {
        return false;
    } else if c >= '\u{10860}' && c <= '\u{10876}' {
        return true;
    } else if c < '\u{10880}' {
        return false;
    } else if c >= '\u{10880}' && c <= '\u{1089E}' {
        return true;
    } else if c < '\u{108E0}' {
        return false;
    } else if c >= '\u{108E0}' && c <= '\u{108F2}' {
        return true;
    } else if c < '\u{108F4}' {
        return false;
    } else if c >= '\u{108F4}' && c <= '\u{108F5}' {
        return true;
    } else if c < '\u{10900}' {
        return false;
    } else if c >= '\u{10900}' && c <= '\u{10915}' {
        return true;
    } else if c < '\u{10920}' {
        return false;
    } else if c >= '\u{10920}' && c <= '\u{10939}' {
        return true;
    } else if c < '\u{10980}' {
        return false;
    } else if c >= '\u{10980}' && c <= '\u{109B7}' {
        return true;
    } else if c < '\u{109BE}' {
        return false;
    } else if c >= '\u{109BE}' && c <= '\u{109BF}' {
        return true;
    } else if c < '\u{10A00}' {
        return false;
    } else if c >= '\u{10A00}' && c <= '\u{10A03}' {
        return true;
    } else if c < '\u{10A05}' {
        return false;
    } else if c >= '\u{10A05}' && c <= '\u{10A06}' {
        return true;
    } else if c < '\u{10A0C}' {
        return false;
    } else if c >= '\u{10A0C}' && c <= '\u{10A13}' {
        return true;
    } else if c < '\u{10A15}' {
        return false;
    } else if c >= '\u{10A15}' && c <= '\u{10A17}' {
        return true;
    } else if c < '\u{10A19}' {
        return false;
    } else if c >= '\u{10A19}' && c <= '\u{10A33}' {
        return true;
    } else if c < '\u{10A38}' {
        return false;
    } else if c >= '\u{10A38}' && c <= '\u{10A3A}' {
        return true;
    } else if c < '\u{10A3F}' {
        return false;
    } else if c >= '\u{10A3F}' && c <= '\u{10A3F}' {
        return true;
    } else if c < '\u{10A60}' {
        return false;
    } else if c >= '\u{10A60}' && c <= '\u{10A7C}' {
        return true;
    } else if c < '\u{10A80}' {
        return false;
    } else if c >= '\u{10A80}' && c <= '\u{10A9C}' {
        return true;
    } else if c < '\u{10AC0}' {
        return false;
    } else if c >= '\u{10AC0}' && c <= '\u{10AC7}' {
        return true;
    } else if c < '\u{10AC9}' {
        return false;
    } else if c >= '\u{10AC9}' && c <= '\u{10AE6}' {
        return true;
    } else if c < '\u{10B00}' {
        return false;
    } else if c >= '\u{10B00}' && c <= '\u{10B35}' {
        return true;
    } else if c < '\u{10B40}' {
        return false;
    } else if c >= '\u{10B40}' && c <= '\u{10B55}' {
        return true;
    } else if c < '\u{10B60}' {
        return false;
    } else if c >= '\u{10B60}' && c <= '\u{10B72}' {
        return true;
    } else if c < '\u{10B80}' {
        return false;
    } else if c >= '\u{10B80}' && c <= '\u{10B91}' {
        return true;
    } else if c < '\u{10C00}' {
        return false;
    } else if c >= '\u{10C00}' && c <= '\u{10C48}' {
        return true;
    } else if c < '\u{10C80}' {
        return false;
    } else if c >= '\u{10C80}' && c <= '\u{10CB2}' {
        return true;
    } else if c < '\u{10CC0}' {
        return false;
    } else if c >= '\u{10CC0}' && c <= '\u{10CF2}' {
        return true;
    } else if c < '\u{11000}' {
        return false;
    } else if c >= '\u{11000}' && c <= '\u{11046}' {
        return true;
    } else if c < '\u{11066}' {
        return false;
    } else if c >= '\u{11066}' && c <= '\u{1106F}' {
        return true;
    } else if c < '\u{1107F}' {
        return false;
    } else if c >= '\u{1107F}' && c <= '\u{110BA}' {
        return true;
    } else if c < '\u{110D0}' {
        return false;
    } else if c >= '\u{110D0}' && c <= '\u{110E8}' {
        return true;
    } else if c < '\u{110F0}' {
        return false;
    } else if c >= '\u{110F0}' && c <= '\u{110F9}' {
        return true;
    } else if c < '\u{11100}' {
        return false;
    } else if c >= '\u{11100}' && c <= '\u{11134}' {
        return true;
    } else if c < '\u{11136}' {
        return false;
    } else if c >= '\u{11136}' && c <= '\u{1113F}' {
        return true;
    } else if c < '\u{11150}' {
        return false;
    } else if c >= '\u{11150}' && c <= '\u{11173}' {
        return true;
    } else if c < '\u{11176}' {
        return false;
    } else if c >= '\u{11176}' && c <= '\u{11176}' {
        return true;
    } else if c < '\u{11180}' {
        return false;
    } else if c >= '\u{11180}' && c <= '\u{111C4}' {
        return true;
    } else if c < '\u{111CA}' {
        return false;
    } else if c >= '\u{111CA}' && c <= '\u{111CC}' {
        return true;
    } else if c < '\u{111D0}' {
        return false;
    } else if c >= '\u{111D0}' && c <= '\u{111DA}' {
        return true;
    } else if c < '\u{111DC}' {
        return false;
    } else if c >= '\u{111DC}' && c <= '\u{111DC}' {
        return true;
    } else if c < '\u{11200}' {
        return false;
    } else if c >= '\u{11200}' && c <= '\u{11211}' {
        return true;
    } else if c < '\u{11213}' {
        return false;
    } else if c >= '\u{11213}' && c <= '\u{11237}' {
        return true;
    } else if c < '\u{1123E}' {
        return false;
    } else if c >= '\u{1123E}' && c <= '\u{1123E}' {
        return true;
    } else if c < '\u{11280}' {
        return false;
    } else if c >= '\u{11280}' && c <= '\u{11286}' {
        return true;
    } else if c < '\u{11288}' {
        return false;
    } else if c >= '\u{11288}' && c <= '\u{11288}' {
        return true;
    } else if c < '\u{1128A}' {
        return false;
    } else if c >= '\u{1128A}' && c <= '\u{1128D}' {
        return true;
    } else if c < '\u{1128F}' {
        return false;
    } else if c >= '\u{1128F}' && c <= '\u{1129D}' {
        return true;
    } else if c < '\u{1129F}' {
        return false;
    } else if c >= '\u{1129F}' && c <= '\u{112A8}' {
        return true;
    } else if c < '\u{112B0}' {
        return false;
    } else if c >= '\u{112B0}' && c <= '\u{112EA}' {
        return true;
    } else if c < '\u{112F0}' {
        return false;
    } else if c >= '\u{112F0}' && c <= '\u{112F9}' {
        return true;
    } else if c < '\u{11300}' {
        return false;
    } else if c >= '\u{11300}' && c <= '\u{11303}' {
        return true;
    } else if c < '\u{11305}' {
        return false;
    } else if c >= '\u{11305}' && c <= '\u{1130C}' {
        return true;
    } else if c < '\u{1130F}' {
        return false;
    } else if c >= '\u{1130F}' && c <= '\u{11310}' {
        return true;
    } else if c < '\u{11313}' {
        return false;
    } else if c >= '\u{11313}' && c <= '\u{11328}' {
        return true;
    } else if c < '\u{1132A}' {
        return false;
    } else if c >= '\u{1132A}' && c <= '\u{11330}' {
        return true;
    } else if c < '\u{11332}' {
        return false;
    } else if c >= '\u{11332}' && c <= '\u{11333}' {
        return true;
    } else if c < '\u{11335}' {
        return false;
    } else if c >= '\u{11335}' && c <= '\u{11339}' {
        return true;
    } else if c < '\u{1133C}' {
        return false;
    } else if c >= '\u{1133C}' && c <= '\u{11344}' {
        return true;
    } else if c < '\u{11347}' {
        return false;
    } else if c >= '\u{11347}' && c <= '\u{11348}' {
        return true;
    } else if c < '\u{1134B}' {
        return false;
    } else if c >= '\u{1134B}' && c <= '\u{1134D}' {
        return true;
    } else if c < '\u{11350}' {
        return false;
    } else if c >= '\u{11350}' && c <= '\u{11350}' {
        return true;
    } else if c < '\u{11357}' {
        return false;
    } else if c >= '\u{11357}' && c <= '\u{11357}' {
        return true;
    } else if c < '\u{1135D}' {
        return false;
    } else if c >= '\u{1135D}' && c <= '\u{11363}' {
        return true;
    } else if c < '\u{11366}' {
        return false;
    } else if c >= '\u{11366}' && c <= '\u{1136C}' {
        return true;
    } else if c < '\u{11370}' {
        return false;
    } else if c >= '\u{11370}' && c <= '\u{11374}' {
        return true;
    } else if c < '\u{11400}' {
        return false;
    } else if c >= '\u{11400}' && c <= '\u{1144A}' {
        return true;
    } else if c < '\u{11450}' {
        return false;
    } else if c >= '\u{11450}' && c <= '\u{11459}' {
        return true;
    } else if c < '\u{11480}' {
        return false;
    } else if c >= '\u{11480}' && c <= '\u{114C5}' {
        return true;
    } else if c < '\u{114C7}' {
        return false;
    } else if c >= '\u{114C7}' && c <= '\u{114C7}' {
        return true;
    } else if c < '\u{114D0}' {
        return false;
    } else if c >= '\u{114D0}' && c <= '\u{114D9}' {
        return true;
    } else if c < '\u{11580}' {
        return false;
    } else if c >= '\u{11580}' && c <= '\u{115B5}' {
        return true;
    } else if c < '\u{115B8}' {
        return false;
    } else if c >= '\u{115B8}' && c <= '\u{115C0}' {
        return true;
    } else if c < '\u{115D8}' {
        return false;
    } else if c >= '\u{115D8}' && c <= '\u{115DD}' {
        return true;
    } else if c < '\u{11600}' {
        return false;
    } else if c >= '\u{11600}' && c <= '\u{11640}' {
        return true;
    } else if c < '\u{11644}' {
        return false;
    } else if c >= '\u{11644}' && c <= '\u{11644}' {
        return true;
    } else if c < '\u{11650}' {
        return false;
    } else if c >= '\u{11650}' && c <= '\u{11659}' {
        return true;
    } else if c < '\u{11680}' {
        return false;
    } else if c >= '\u{11680}' && c <= '\u{116B7}' {
        return true;
    } else if c < '\u{116C0}' {
        return false;
    } else if c >= '\u{116C0}' && c <= '\u{116C9}' {
        return true;
    } else if c < '\u{11700}' {
        return false;
    } else if c >= '\u{11700}' && c <= '\u{11719}' {
        return true;
    } else if c < '\u{1171D}' {
        return false;
    } else if c >= '\u{1171D}' && c <= '\u{1172B}' {
        return true;
    } else if c < '\u{11730}' {
        return false;
    } else if c >= '\u{11730}' && c <= '\u{11739}' {
        return true;
    } else if c < '\u{118A0}' {
        return false;
    } else if c >= '\u{118A0}' && c <= '\u{118E9}' {
        return true;
    } else if c < '\u{118FF}' {
        return false;
    } else if c >= '\u{118FF}' && c <= '\u{118FF}' {
        return true;
    } else if c < '\u{11AC0}' {
        return false;
    } else if c >= '\u{11AC0}' && c <= '\u{11AF8}' {
        return true;
    } else if c < '\u{11C00}' {
        return false;
    } else if c >= '\u{11C00}' && c <= '\u{11C08}' {
        return true;
    } else if c < '\u{11C0A}' {
        return false;
    } else if c >= '\u{11C0A}' && c <= '\u{11C36}' {
        return true;
    } else if c < '\u{11C38}' {
        return false;
    } else if c >= '\u{11C38}' && c <= '\u{11C40}' {
        return true;
    } else if c < '\u{11C50}' {
        return false;
    } else if c >= '\u{11C50}' && c <= '\u{11C59}' {
        return true;
    } else if c < '\u{11C72}' {
        return false;
    } else if c >= '\u{11C72}' && c <= '\u{11C8F}' {
        return true;
    } else if c < '\u{11C92}' {
        return false;
    } else if c >= '\u{11C92}' && c <= '\u{11CA7}' {
        return true;
    } else if c < '\u{11CA9}' {
        return false;
    } else if c >= '\u{11CA9}' && c <= '\u{11CB6}' {
        return true;
    } else if c < '\u{12000}' {
        return false;
    } else if c >= '\u{12000}' && c <= '\u{12399}' {
        return true;
    } else if c < '\u{12400}' {
        return false;
    } else if c >= '\u{12400}' && c <= '\u{1246E}' {
        return true;
    } else if c < '\u{12480}' {
        return false;
    } else if c >= '\u{12480}' && c <= '\u{12543}' {
        return true;
    } else if c < '\u{13000}' {
        return false;
    } else if c >= '\u{13000}' && c <= '\u{1342E}' {
        return true;
    } else if c < '\u{14400}' {
        return false;
    } else if c >= '\u{14400}' && c <= '\u{14646}' {
        return true;
    } else if c < '\u{16800}' {
        return false;
    } else if c >= '\u{16800}' && c <= '\u{16A38}' {
        return true;
    } else if c < '\u{16A40}' {
        return false;
    } else if c >= '\u{16A40}' && c <= '\u{16A5E}' {
        return true;
    } else if c < '\u{16A60}' {
        return false;
    } else if c >= '\u{16A60}' && c <= '\u{16A69}' {
        return true;
    } else if c < '\u{16AD0}' {
        return false;
    } else if c >= '\u{16AD0}' && c <= '\u{16AED}' {
        return true;
    } else if c < '\u{16AF0}' {
        return false;
    } else if c >= '\u{16AF0}' && c <= '\u{16AF4}' {
        return true;
    } else if c < '\u{16B00}' {
        return false;
    } else if c >= '\u{16B00}' && c <= '\u{16B36}' {
        return true;
    } else if c < '\u{16B40}' {
        return false;
    } else if c >= '\u{16B40}' && c <= '\u{16B43}' {
        return true;
    } else if c < '\u{16B50}' {
        return false;
    } else if c >= '\u{16B50}' && c <= '\u{16B59}' {
        return true;
    } else if c < '\u{16B63}' {
        return false;
    } else if c >= '\u{16B63}' && c <= '\u{16B77}' {
        return true;
    } else if c < '\u{16B7D}' {
        return false;
    } else if c >= '\u{16B7D}' && c <= '\u{16B8F}' {
        return true;
    } else if c < '\u{16F00}' {
        return false;
    } else if c >= '\u{16F00}' && c <= '\u{16F44}' {
        return true;
    } else if c < '\u{16F50}' {
        return false;
    } else if c >= '\u{16F50}' && c <= '\u{16F7E}' {
        return true;
    } else if c < '\u{16F8F}' {
        return false;
    } else if c >= '\u{16F8F}' && c <= '\u{16F9F}' {
        return true;
    } else if c < '\u{16FE0}' {
        return false;
    } else if c >= '\u{16FE0}' && c <= '\u{16FE0}' {
        return true;
    } else if c < '\u{17000}' {
        return false;
    } else if c >= '\u{17000}' && c <= '\u{187EC}' {
        return true;
    } else if c < '\u{18800}' {
        return false;
    } else if c >= '\u{18800}' && c <= '\u{18AF2}' {
        return true;
    } else if c < '\u{1B000}' {
        return false;
    } else if c >= '\u{1B000}' && c <= '\u{1B001}' {
        return true;
    } else if c < '\u{1BC00}' {
        return false;
    } else if c >= '\u{1BC00}' && c <= '\u{1BC6A}' {
        return true;
    } else if c < '\u{1BC70}' {
        return false;
    } else if c >= '\u{1BC70}' && c <= '\u{1BC7C}' {
        return true;
    } else if c < '\u{1BC80}' {
        return false;
    } else if c >= '\u{1BC80}' && c <= '\u{1BC88}' {
        return true;
    } else if c < '\u{1BC90}' {
        return false;
    } else if c >= '\u{1BC90}' && c <= '\u{1BC99}' {
        return true;
    } else if c < '\u{1BC9D}' {
        return false;
    } else if c >= '\u{1BC9D}' && c <= '\u{1BC9E}' {
        return true;
    } else if c < '\u{1D165}' {
        return false;
    } else if c >= '\u{1D165}' && c <= '\u{1D169}' {
        return true;
    } else if c < '\u{1D16D}' {
        return false;
    } else if c >= '\u{1D16D}' && c <= '\u{1D172}' {
        return true;
    } else if c < '\u{1D17B}' {
        return false;
    } else if c >= '\u{1D17B}' && c <= '\u{1D182}' {
        return true;
    } else if c < '\u{1D185}' {
        return false;
    } else if c >= '\u{1D185}' && c <= '\u{1D18B}' {
        return true;
    } else if c < '\u{1D1AA}' {
        return false;
    } else if c >= '\u{1D1AA}' && c <= '\u{1D1AD}' {
        return true;
    } else if c < '\u{1D242}' {
        return false;
    } else if c >= '\u{1D242}' && c <= '\u{1D244}' {
        return true;
    } else if c < '\u{1D400}' {
        return false;
    } else if c >= '\u{1D400}' && c <= '\u{1D454}' {
        return true;
    } else if c < '\u{1D456}' {
        return false;
    } else if c >= '\u{1D456}' && c <= '\u{1D49C}' {
        return true;
    } else if c < '\u{1D49E}' {
        return false;
    } else if c >= '\u{1D49E}' && c <= '\u{1D49F}' {
        return true;
    } else if c < '\u{1D4A2}' {
        return false;
    } else if c >= '\u{1D4A2}' && c <= '\u{1D4A2}' {
        return true;
    } else if c < '\u{1D4A5}' {
        return false;
    } else if c >= '\u{1D4A5}' && c <= '\u{1D4A6}' {
        return true;
    } else if c < '\u{1D4A9}' {
        return false;
    } else if c >= '\u{1D4A9}' && c <= '\u{1D4AC}' {
        return true;
    } else if c < '\u{1D4AE}' {
        return false;
    } else if c >= '\u{1D4AE}' && c <= '\u{1D4B9}' {
        return true;
    } else if c < '\u{1D4BB}' {
        return false;
    } else if c >= '\u{1D4BB}' && c <= '\u{1D4BB}' {
        return true;
    } else if c < '\u{1D4BD}' {
        return false;
    } else if c >= '\u{1D4BD}' && c <= '\u{1D4C3}' {
        return true;
    } else if c < '\u{1D4C5}' {
        return false;
    } else if c >= '\u{1D4C5}' && c <= '\u{1D505}' {
        return true;
    } else if c < '\u{1D507}' {
        return false;
    } else if c >= '\u{1D507}' && c <= '\u{1D50A}' {
        return true;
    } else if c < '\u{1D50D}' {
        return false;
    } else if c >= '\u{1D50D}' && c <= '\u{1D514}' {
        return true;
    } else if c < '\u{1D516}' {
        return false;
    } else if c >= '\u{1D516}' && c <= '\u{1D51C}' {
        return true;
    } else if c < '\u{1D51E}' {
        return false;
    } else if c >= '\u{1D51E}' && c <= '\u{1D539}' {
        return true;
    } else if c < '\u{1D53B}' {
        return false;
    } else if c >= '\u{1D53B}' && c <= '\u{1D53E}' {
        return true;
    } else if c < '\u{1D540}' {
        return false;
    } else if c >= '\u{1D540}' && c <= '\u{1D544}' {
        return true;
    } else if c < '\u{1D546}' {
        return false;
    } else if c >= '\u{1D546}' && c <= '\u{1D546}' {
        return true;
    } else if c < '\u{1D54A}' {
        return false;
    } else if c >= '\u{1D54A}' && c <= '\u{1D550}' {
        return true;
    } else if c < '\u{1D552}' {
        return false;
    } else if c >= '\u{1D552}' && c <= '\u{1D6A5}' {
        return true;
    } else if c < '\u{1D6A8}' {
        return false;
    } else if c >= '\u{1D6A8}' && c <= '\u{1D6C0}' {
        return true;
    } else if c < '\u{1D6C2}' {
        return false;
    } else if c >= '\u{1D6C2}' && c <= '\u{1D6DA}' {
        return true;
    } else if c < '\u{1D6DC}' {
        return false;
    } else if c >= '\u{1D6DC}' && c <= '\u{1D6FA}' {
        return true;
    } else if c < '\u{1D6FC}' {
        return false;
    } else if c >= '\u{1D6FC}' && c <= '\u{1D714}' {
        return true;
    } else if c < '\u{1D716}' {
        return false;
    } else if c >= '\u{1D716}' && c <= '\u{1D734}' {
        return true;
    } else if c < '\u{1D736}' {
        return false;
    } else if c >= '\u{1D736}' && c <= '\u{1D74E}' {
        return true;
    } else if c < '\u{1D750}' {
        return false;
    } else if c >= '\u{1D750}' && c <= '\u{1D76E}' {
        return true;
    } else if c < '\u{1D770}' {
        return false;
    } else if c >= '\u{1D770}' && c <= '\u{1D788}' {
        return true;
    } else if c < '\u{1D78A}' {
        return false;
    } else if c >= '\u{1D78A}' && c <= '\u{1D7A8}' {
        return true;
    } else if c < '\u{1D7AA}' {
        return false;
    } else if c >= '\u{1D7AA}' && c <= '\u{1D7C2}' {
        return true;
    } else if c < '\u{1D7C4}' {
        return false;
    } else if c >= '\u{1D7C4}' && c <= '\u{1D7CB}' {
        return true;
    } else if c < '\u{1D7CE}' {
        return false;
    } else if c >= '\u{1D7CE}' && c <= '\u{1D7FF}' {
        return true;
    } else if c < '\u{1DA00}' {
        return false;
    } else if c >= '\u{1DA00}' && c <= '\u{1DA36}' {
        return true;
    } else if c < '\u{1DA3B}' {
        return false;
    } else if c >= '\u{1DA3B}' && c <= '\u{1DA6C}' {
        return true;
    } else if c < '\u{1DA75}' {
        return false;
    } else if c >= '\u{1DA75}' && c <= '\u{1DA75}' {
        return true;
    } else if c < '\u{1DA84}' {
        return false;
    } else if c >= '\u{1DA84}' && c <= '\u{1DA84}' {
        return true;
    } else if c < '\u{1DA9B}' {
        return false;
    } else if c >= '\u{1DA9B}' && c <= '\u{1DA9F}' {
        return true;
    } else if c < '\u{1DAA1}' {
        return false;
    } else if c >= '\u{1DAA1}' && c <= '\u{1DAAF}' {
        return true;
    } else if c < '\u{1E000}' {
        return false;
    } else if c >= '\u{1E000}' && c <= '\u{1E006}' {
        return true;
    } else if c < '\u{1E008}' {
        return false;
    } else if c >= '\u{1E008}' && c <= '\u{1E018}' {
        return true;
    } else if c < '\u{1E01B}' {
        return false;
    } else if c >= '\u{1E01B}' && c <= '\u{1E021}' {
        return true;
    } else if c < '\u{1E023}' {
        return false;
    } else if c >= '\u{1E023}' && c <= '\u{1E024}' {
        return true;
    } else if c < '\u{1E026}' {
        return false;
    } else if c >= '\u{1E026}' && c <= '\u{1E02A}' {
        return true;
    } else if c < '\u{1E800}' {
        return false;
    } else if c >= '\u{1E800}' && c <= '\u{1E8C4}' {
        return true;
    } else if c < '\u{1E8D0}' {
        return false;
    } else if c >= '\u{1E8D0}' && c <= '\u{1E8D6}' {
        return true;
    } else if c < '\u{1E900}' {
        return false;
    } else if c >= '\u{1E900}' && c <= '\u{1E94A}' {
        return true;
    } else if c < '\u{1E950}' {
        return false;
    } else if c >= '\u{1E950}' && c <= '\u{1E959}' {
        return true;
    } else if c < '\u{1EE00}' {
        return false;
    } else if c >= '\u{1EE00}' && c <= '\u{1EE03}' {
        return true;
    } else if c < '\u{1EE05}' {
        return false;
    } else if c >= '\u{1EE05}' && c <= '\u{1EE1F}' {
        return true;
    } else if c < '\u{1EE21}' {
        return false;
    } else if c >= '\u{1EE21}' && c <= '\u{1EE22}' {
        return true;
    } else if c < '\u{1EE24}' {
        return false;
    } else if c >= '\u{1EE24}' && c <= '\u{1EE24}' {
        return true;
    } else if c < '\u{1EE27}' {
        return false;
    } else if c >= '\u{1EE27}' && c <= '\u{1EE27}' {
        return true;
    } else if c < '\u{1EE29}' {
        return false;
    } else if c >= '\u{1EE29}' && c <= '\u{1EE32}' {
        return true;
    } else if c < '\u{1EE34}' {
        return false;
    } else if c >= '\u{1EE34}' && c <= '\u{1EE37}' {
        return true;
    } else if c < '\u{1EE39}' {
        return false;
    } else if c >= '\u{1EE39}' && c <= '\u{1EE39}' {
        return true;
    } else if c < '\u{1EE3B}' {
        return false;
    } else if c >= '\u{1EE3B}' && c <= '\u{1EE3B}' {
        return true;
    } else if c < '\u{1EE42}' {
        return false;
    } else if c >= '\u{1EE42}' && c <= '\u{1EE42}' {
        return true;
    } else if c < '\u{1EE47}' {
        return false;
    } else if c >= '\u{1EE47}' && c <= '\u{1EE47}' {
        return true;
    } else if c < '\u{1EE49}' {
        return false;
    } else if c >= '\u{1EE49}' && c <= '\u{1EE49}' {
        return true;
    } else if c < '\u{1EE4B}' {
        return false;
    } else if c >= '\u{1EE4B}' && c <= '\u{1EE4B}' {
        return true;
    } else if c < '\u{1EE4D}' {
        return false;
    } else if c >= '\u{1EE4D}' && c <= '\u{1EE4F}' {
        return true;
    } else if c < '\u{1EE51}' {
        return false;
    } else if c >= '\u{1EE51}' && c <= '\u{1EE52}' {
        return true;
    } else if c < '\u{1EE54}' {
        return false;
    } else if c >= '\u{1EE54}' && c <= '\u{1EE54}' {
        return true;
    } else if c < '\u{1EE57}' {
        return false;
    } else if c >= '\u{1EE57}' && c <= '\u{1EE57}' {
        return true;
    } else if c < '\u{1EE59}' {
        return false;
    } else if c >= '\u{1EE59}' && c <= '\u{1EE59}' {
        return true;
    } else if c < '\u{1EE5B}' {
        return false;
    } else if c >= '\u{1EE5B}' && c <= '\u{1EE5B}' {
        return true;
    } else if c < '\u{1EE5D}' {
        return false;
    } else if c >= '\u{1EE5D}' && c <= '\u{1EE5D}' {
        return true;
    } else if c < '\u{1EE5F}' {
        return false;
    } else if c >= '\u{1EE5F}' && c <= '\u{1EE5F}' {
        return true;
    } else if c < '\u{1EE61}' {
        return false;
    } else if c >= '\u{1EE61}' && c <= '\u{1EE62}' {
        return true;
    } else if c < '\u{1EE64}' {
        return false;
    } else if c >= '\u{1EE64}' && c <= '\u{1EE64}' {
        return true;
    } else if c < '\u{1EE67}' {
        return false;
    } else if c >= '\u{1EE67}' && c <= '\u{1EE6A}' {
        return true;
    } else if c < '\u{1EE6C}' {
        return false;
    } else if c >= '\u{1EE6C}' && c <= '\u{1EE72}' {
        return true;
    } else if c < '\u{1EE74}' {
        return false;
    } else if c >= '\u{1EE74}' && c <= '\u{1EE77}' {
        return true;
    } else if c < '\u{1EE79}' {
        return false;
    } else if c >= '\u{1EE79}' && c <= '\u{1EE7C}' {
        return true;
    } else if c < '\u{1EE7E}' {
        return false;
    } else if c >= '\u{1EE7E}' && c <= '\u{1EE7E}' {
        return true;
    } else if c < '\u{1EE80}' {
        return false;
    } else if c >= '\u{1EE80}' && c <= '\u{1EE89}' {
        return true;
    } else if c < '\u{1EE8B}' {
        return false;
    } else if c >= '\u{1EE8B}' && c <= '\u{1EE9B}' {
        return true;
    } else if c < '\u{1EEA1}' {
        return false;
    } else if c >= '\u{1EEA1}' && c <= '\u{1EEA3}' {
        return true;
    } else if c < '\u{1EEA5}' {
        return false;
    } else if c >= '\u{1EEA5}' && c <= '\u{1EEA9}' {
        return true;
    } else if c < '\u{1EEAB}' {
        return false;
    } else if c >= '\u{1EEAB}' && c <= '\u{1EEBB}' {
        return true;
    } else if c < '\u{20000}' {
        return false;
    } else if c >= '\u{20000}' && c <= '\u{2A6D6}' {
        return true;
    } else if c < '\u{2A700}' {
        return false;
    } else if c >= '\u{2A700}' && c <= '\u{2B734}' {
        return true;
    } else if c < '\u{2B740}' {
        return false;
    } else if c >= '\u{2B740}' && c <= '\u{2B81D}' {
        return true;
    } else if c < '\u{2B820}' {
        return false;
    } else if c >= '\u{2B820}' && c <= '\u{2CEA1}' {
        return true;
    } else if c < '\u{2F800}' {
        return false;
    } else if c >= '\u{2F800}' && c <= '\u{2FA1D}' {
        return true;
    } else if c < '\u{E0100}' {
        return false;
    } else if c >= '\u{E0100}' && c <= '\u{E01EF}' {
        return true;
    }
    false
}

#[inline]
pub(crate) fn is_other_whitespace(c: char) -> bool {
    if c < '\u{2000}' {
        false
    } else if c >= '\u{2000}' && c <= '\u{200A}' {
        true
    } else if c < '\u{202F}' {
        false
    } else if c == '\u{202F}' {
        true
    } else if c < '\u{205F}' {
        false
    } else if c == '\u{205F}' {
        true
    } else if c < '\u{3000}' {
        false
    } else if c == '\u{3000}' {
        true
    } else {
        false
    }
}
