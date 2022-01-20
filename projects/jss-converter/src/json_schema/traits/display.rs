use std::fmt::Write;
use text_utils::indent;
use super::*;


impl Display for JssSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.description(f)?;

        match self.kind {
            JssKind::Scheme => { f.write_str("schema ") }
            JssKind::Property => {f.write_str("property ")}
            JssKind::PropertyTop => {f.write_str(".")}
            JssKind::Definition => {f.write_str("define ")}
        }?;

        self.head(f)?;
        for (key, value) in &self.keywords {
            writeln!(f, "{}", indent(format!("{}: {:#?}", key, value), 4))?
        }
        for (key, value) in &self.annotation {
            writeln!(f, "{}", indent(format!("{}: {:#?}", key, value), 4))?
        }
        match self.kind {
            JssKind::Scheme => {}
            _ => {
                for (_, value) in &self.properties {
                    writeln!(f, "{}", indent(format!("{:#?}", value), 4))?
                }
            }
        }
        f.write_str("}\n")?;
        if let JssKind::Scheme = self.kind {
            for (key, value) in &self.definition {
                writeln!(f, "{}", format!("{}: {:#?}", key, value))?
            }
            for (_, value) in &self.properties {
                Display::fmt(value, f)?
            }
        }
        Ok(())
    }
}

impl JssSchema {
    #[inline]
    fn head(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            None => { f.write_char('_')? }
            Some(s) => { f.write_str(s)? }
        }
        if self.typing != JssType::Undefined {
            f.write_str(&self.typing.to_string())?
        }
        f.write_str(" {\n")?;
        Ok(())
    }
    #[inline]
    fn description(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(s) = &self.description {
            for line in s.lines() {
                writeln!(f, "/// {}", line)?
            }
        }
        Ok(())
    }
}

impl Display for JssValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => { unimplemented!() }
            Self::Bool(_) => { unimplemented!() }
            Self::Number(_) => { unimplemented!() }
            Self::String(v) => { Debug::fmt(v, f) }
            Self::Url(_) => { unimplemented!() }
            Self::Regex(_) => { unimplemented!() }
            Self::Array(v) => {
                Debug::fmt(v, f)
            }
            Self::Object(_) => { unimplemented!() }
        }
    }
}

impl Display for JssType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undefined => { write!(f, "undefined") }
            Self::Anything => { write!(f, ": anything") }
            Self::Nothing => { write!(f, ": nothing") }
            Self::String(_) => { write!(f, ": string") }
            Self::Number => { write!(f, ": number") }
            Self::Object => { write!(f, ": object") }
            Self::Reference(v) => { write!(f, ": {}", v) }
        }
    }
}
