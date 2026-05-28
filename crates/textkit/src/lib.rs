pub mod error;

pub use error::*;

use graphitepdf_font::{FontDescriptor, FontSource, FontStore, StandardFont};
use graphitepdf_primitives::Pt;
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq)]
pub struct TextSpan {
    content: String,
    font: Option<FontDescriptor>,
    font_size: Pt,
}

impl TextSpan {
    pub fn new(content: impl Into<String>) -> Result<Self> {
        let content = content.into();
        if content.trim().is_empty() {
            return Err(Error::EmptyText);
        }

        Ok(Self {
            content,
            font: None,
            font_size: Pt::new(12.0),
        })
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn font(&self) -> Option<&FontDescriptor> {
        self.font.as_ref()
    }

    pub const fn font_size(&self) -> Pt {
        self.font_size
    }

    pub fn with_font(mut self, font: FontDescriptor) -> Self {
        self.font = Some(font);
        self
    }

    pub fn with_font_size(mut self, font_size: Pt) -> Result<Self> {
        if font_size.value() <= 0.0 {
            return Err(Error::InvalidFontSize {
                size: font_size.value(),
            });
        }

        self.font_size = font_size;
        Ok(self)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextBlock {
    spans: Vec<TextSpan>,
}

impl TextBlock {
    pub fn new(spans: impl IntoIterator<Item = TextSpan>) -> Self {
        Self {
            spans: spans.into_iter().collect(),
        }
    }

    pub fn push(&mut self, span: TextSpan) {
        self.spans.push(span);
    }

    pub fn spans(&self) -> &[TextSpan] {
        &self.spans
    }

    pub fn plain_text(&self) -> String {
        self.spans
            .iter()
            .map(TextSpan::content)
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn is_empty(&self) -> bool {
        self.spans.is_empty()
    }
}

impl From<TextSpan> for TextBlock {
    fn from(value: TextSpan) -> Self {
        Self { spans: vec![value] }
    }
}

impl TextBlock {
    pub fn to_attributed_string(&self) -> Result<AttributedString> {
        AttributedString::try_from(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextRange {
    start: usize,
    end: usize,
}

impl TextRange {
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub const fn start(self) -> usize {
        self.start
    }

    pub const fn end(self) -> usize {
        self.end
    }

    pub const fn len(self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub const fn is_empty(self) -> bool {
        self.start >= self.end
    }

    fn validate_for(self, text: &str) -> Result<()> {
        if self.start > self.end || self.end > text.len() {
            return Err(Error::InvalidTextRange {
                start: self.start,
                end: self.end,
                len: text.len(),
            });
        }

        if !text.is_char_boundary(self.start) || !text.is_char_boundary(self.end) {
            return Err(Error::NonCharacterBoundaryRange {
                start: self.start,
                end: self.end,
            });
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextDirection {
    #[default]
    Ltr,
    Rtl,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Script {
    #[default]
    Common,
    Latin,
    Arabic,
    Hebrew,
    Cyrillic,
    Han,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextDecorationKind {
    Underline,
    Overline,
    LineThrough,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextDecoration {
    kind: TextDecorationKind,
    thickness: Option<Pt>,
    offset: Option<Pt>,
}

impl TextDecoration {
    pub const fn new(kind: TextDecorationKind) -> Self {
        Self {
            kind,
            thickness: None,
            offset: None,
        }
    }

    pub const fn kind(&self) -> TextDecorationKind {
        self.kind
    }

    pub const fn thickness(&self) -> Option<Pt> {
        self.thickness
    }

    pub const fn offset(&self) -> Option<Pt> {
        self.offset
    }

    pub fn with_thickness(mut self, thickness: Pt) -> Self {
        self.thickness = Some(thickness);
        self
    }

    pub fn with_offset(mut self, offset: Pt) -> Self {
        self.offset = Some(offset);
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextAttributes {
    font: Option<FontDescriptor>,
    font_size: Pt,
    letter_spacing: Pt,
    language: Option<String>,
    direction: Option<TextDirection>,
    decorations: Vec<TextDecoration>,
}

impl Default for TextAttributes {
    fn default() -> Self {
        Self {
            font: None,
            font_size: Pt::new(12.0),
            letter_spacing: Pt::zero(),
            language: None,
            direction: None,
            decorations: Vec::new(),
        }
    }
}

impl TextAttributes {
    pub fn font(&self) -> Option<&FontDescriptor> {
        self.font.as_ref()
    }

    pub const fn font_size(&self) -> Pt {
        self.font_size
    }

    pub const fn letter_spacing(&self) -> Pt {
        self.letter_spacing
    }

    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    pub const fn direction(&self) -> Option<TextDirection> {
        self.direction
    }

    pub fn decorations(&self) -> &[TextDecoration] {
        &self.decorations
    }

    pub fn with_font(mut self, font: FontDescriptor) -> Self {
        self.font = Some(font);
        self
    }

    pub fn with_font_size(mut self, font_size: Pt) -> Result<Self> {
        if font_size.value() <= 0.0 {
            return Err(Error::InvalidFontSize {
                size: font_size.value(),
            });
        }

        self.font_size = font_size;
        Ok(self)
    }

    pub fn with_letter_spacing(mut self, letter_spacing: Pt) -> Self {
        self.letter_spacing = letter_spacing;
        self
    }

    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn with_direction(mut self, direction: TextDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn with_decoration(mut self, decoration: TextDecoration) -> Self {
        self.decorations.push(decoration);
        self
    }
}

impl From<&TextSpan> for TextAttributes {
    fn from(value: &TextSpan) -> Self {
        let mut attributes = TextAttributes::default();
        if let Some(font) = value.font.clone() {
            attributes = attributes.with_font(font);
        }
        attributes.font_size = value.font_size;
        attributes
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AttributeRun {
    range: TextRange,
    attributes: TextAttributes,
}

impl AttributeRun {
    pub const fn new(range: TextRange, attributes: TextAttributes) -> Self {
        Self { range, attributes }
    }

    pub const fn range(&self) -> TextRange {
        self.range
    }

    pub const fn attributes(&self) -> &TextAttributes {
        &self.attributes
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AttributedString {
    text: String,
    default_attributes: TextAttributes,
    runs: Vec<AttributeRun>,
}

impl AttributedString {
    pub fn new(text: impl Into<String>) -> Result<Self> {
        let text = text.into();
        if text.is_empty() {
            return Err(Error::EmptyText);
        }

        Ok(Self {
            text,
            default_attributes: TextAttributes::default(),
            runs: Vec::new(),
        })
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn default_attributes(&self) -> &TextAttributes {
        &self.default_attributes
    }

    pub fn runs(&self) -> &[AttributeRun] {
        &self.runs
    }

    pub fn with_default_attributes(mut self, attributes: TextAttributes) -> Result<Self> {
        validate_font_size(attributes.font_size())?;
        self.default_attributes = attributes;
        Ok(self)
    }

    pub fn push_run(&mut self, range: TextRange, attributes: TextAttributes) -> Result<()> {
        range.validate_for(&self.text)?;
        validate_font_size(attributes.font_size())?;
        self.runs.push(AttributeRun::new(range, attributes));
        Ok(())
    }

    pub fn with_run(mut self, range: TextRange, attributes: TextAttributes) -> Result<Self> {
        self.push_run(range, attributes)?;
        Ok(self)
    }

    fn canonical_runs(&self) -> Result<Vec<CanonicalRun>> {
        let mut boundaries = vec![0, self.text.len()];
        for run in &self.runs {
            run.range.validate_for(&self.text)?;
            boundaries.push(run.range.start());
            boundaries.push(run.range.end());
        }
        boundaries.sort_unstable();
        boundaries.dedup();

        let mut runs = Vec::new();
        for window in boundaries.windows(2) {
            let range = TextRange::new(window[0], window[1]);
            if range.is_empty() {
                continue;
            }

            let mut attributes = self.default_attributes.clone();
            for run in &self.runs {
                if run.range.start() <= range.start() && range.end() <= run.range.end() {
                    attributes = run.attributes.clone();
                }
            }

            let text = self.text[range.start()..range.end()].to_string();
            runs.push(CanonicalRun {
                range,
                text,
                attributes,
            });
        }

        Ok(runs)
    }
}

impl TryFrom<&TextBlock> for AttributedString {
    type Error = Error;

    fn try_from(value: &TextBlock) -> Result<Self> {
        let text = value.plain_text();
        if text.is_empty() {
            return Err(Error::EmptyText);
        }

        let mut attributed = AttributedString::new(text)?;
        let mut start = 0;
        for span in value.spans() {
            let end = start + span.content().len();
            attributed.push_run(TextRange::new(start, end), TextAttributes::from(span))?;
            start = end;
        }
        Ok(attributed)
    }
}

impl TryFrom<TextBlock> for AttributedString {
    type Error = Error;

    fn try_from(value: TextBlock) -> Result<Self> {
        Self::try_from(&value)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TextRect {
    pub x: Pt,
    pub y: Pt,
    pub width: Pt,
    pub height: Pt,
}

impl TextRect {
    pub const fn new(x: Pt, y: Pt, width: Pt, height: Pt) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub const fn from_values(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self::new(Pt::new(x), Pt::new(y), Pt::new(width), Pt::new(height))
    }

    pub const fn right(&self) -> Pt {
        Pt::new(self.x.value() + self.width.value())
    }

    pub const fn bottom(&self) -> Pt {
        Pt::new(self.y.value() + self.height.value())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextContainer {
    rect: TextRect,
    max_lines: Option<usize>,
}

impl TextContainer {
    pub fn new(rect: TextRect) -> Result<Self> {
        if rect.width.value() <= 0.0 || rect.height.value() <= 0.0 {
            return Err(Error::InvalidTextContainer {
                width: rect.width.value(),
                height: rect.height.value(),
            });
        }

        Ok(Self {
            rect,
            max_lines: None,
        })
    }

    pub const fn rect(&self) -> TextRect {
        self.rect
    }

    pub const fn max_lines(&self) -> Option<usize> {
        self.max_lines
    }

    pub fn with_max_lines(mut self, max_lines: usize) -> Self {
        self.max_lines = Some(max_lines);
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BidiMode {
    #[default]
    Auto,
    LeftToRight,
    RightToLeft,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LineBreaking {
    #[default]
    WordBoundary,
    CharacterBoundary,
    WordBoundaryOrCharacter,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Justification {
    #[default]
    Start,
    End,
    Center,
    Full,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScriptItemization {
    None,
    #[default]
    Heuristic,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextDecorationMode {
    Suppress,
    #[default]
    Preserve,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FontSubstitution {
    Disabled,
    FallbackFamilies(Vec<FontDescriptor>),
    #[default]
    BestEffort,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WordHyphenation {
    Disabled,
    Enabled { min_word_chars: usize, marker: char },
}

impl Default for WordHyphenation {
    fn default() -> Self {
        Self::Enabled {
            min_word_chars: 8,
            marker: '-',
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextEngineConfig {
    pub bidi: BidiMode,
    pub line_breaking: LineBreaking,
    pub justification: Justification,
    pub font_substitution: FontSubstitution,
    pub script_itemization: ScriptItemization,
    pub text_decoration_mode: TextDecorationMode,
    pub word_hyphenation: WordHyphenation,
    pub default_font: FontDescriptor,
}

impl Default for TextEngineConfig {
    fn default() -> Self {
        Self {
            bidi: BidiMode::Auto,
            line_breaking: LineBreaking::WordBoundaryOrCharacter,
            justification: Justification::Start,
            font_substitution: FontSubstitution::BestEffort,
            script_itemization: ScriptItemization::Heuristic,
            text_decoration_mode: TextDecorationMode::Preserve,
            word_hyphenation: WordHyphenation::default(),
            default_font: FontDescriptor::new(StandardFont::Helvetica.family_name()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextRun {
    range: TextRange,
    text: String,
    requested_font: Option<FontDescriptor>,
    resolved_font: FontDescriptor,
    font_source: Option<FontSource>,
    font_size: Pt,
    letter_spacing: Pt,
    language: Option<String>,
    direction: TextDirection,
    script: Script,
    decorations: Vec<TextDecoration>,
}

impl TextRun {
    pub const fn range(&self) -> TextRange {
        self.range
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn requested_font(&self) -> Option<&FontDescriptor> {
        self.requested_font.as_ref()
    }

    pub const fn resolved_font(&self) -> &FontDescriptor {
        &self.resolved_font
    }

    pub const fn font_source(&self) -> Option<&FontSource> {
        self.font_source.as_ref()
    }

    pub const fn font_size(&self) -> Pt {
        self.font_size
    }

    pub const fn letter_spacing(&self) -> Pt {
        self.letter_spacing
    }

    pub fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    pub const fn direction(&self) -> TextDirection {
        self.direction
    }

    pub const fn script(&self) -> Script {
        self.script
    }

    pub fn decorations(&self) -> &[TextDecoration] {
        &self.decorations
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextFragment {
    range: TextRange,
    text: String,
    rect: TextRect,
    baseline: Pt,
    direction: TextDirection,
    script: Script,
    font: FontDescriptor,
    font_size: Pt,
    decorations: Vec<TextDecoration>,
    inserted_hyphen: bool,
    whitespace: bool,
}

impl TextFragment {
    pub const fn range(&self) -> TextRange {
        self.range
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub const fn rect(&self) -> TextRect {
        self.rect
    }

    pub const fn baseline(&self) -> Pt {
        self.baseline
    }

    pub const fn direction(&self) -> TextDirection {
        self.direction
    }

    pub const fn script(&self) -> Script {
        self.script
    }

    pub const fn font(&self) -> &FontDescriptor {
        &self.font
    }

    pub const fn font_size(&self) -> Pt {
        self.font_size
    }

    pub fn decorations(&self) -> &[TextDecoration] {
        &self.decorations
    }

    pub const fn inserted_hyphen(&self) -> bool {
        self.inserted_hyphen
    }

    pub const fn is_whitespace(&self) -> bool {
        self.whitespace
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineFragment {
    rect: TextRect,
    baseline: Pt,
    direction: TextDirection,
    justification: Justification,
    fragments: Vec<TextFragment>,
}

impl LineFragment {
    pub const fn rect(&self) -> TextRect {
        self.rect
    }

    pub const fn baseline(&self) -> Pt {
        self.baseline
    }

    pub const fn direction(&self) -> TextDirection {
        self.direction
    }

    pub const fn justification(&self) -> Justification {
        self.justification
    }

    pub fn fragments(&self) -> &[TextFragment] {
        &self.fragments
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextLayout {
    lines: Vec<LineFragment>,
    runs: Vec<TextRun>,
    bounds: TextRect,
    overflowed: bool,
}

impl TextLayout {
    pub fn lines(&self) -> &[LineFragment] {
        &self.lines
    }

    pub fn runs(&self) -> &[TextRun] {
        &self.runs
    }

    pub const fn bounds(&self) -> TextRect {
        self.bounds
    }

    pub const fn overflowed(&self) -> bool {
        self.overflowed
    }

    pub fn fragments(&self) -> impl Iterator<Item = &TextFragment> {
        self.lines.iter().flat_map(|line| line.fragments.iter())
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextEngine {
    config: TextEngineConfig,
}

impl TextEngine {
    pub fn new(config: TextEngineConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &TextEngineConfig {
        &self.config
    }

    pub fn layout(
        &self,
        attributed: &AttributedString,
        container: &TextContainer,
        font_store: Option<&FontStore>,
    ) -> Result<TextLayout> {
        let runs = self.resolve_runs(attributed, font_store)?;
        self.layout_runs(runs, *container, font_store)
    }

    pub fn layout_text_block(
        &self,
        block: &TextBlock,
        container: &TextContainer,
        font_store: Option<&FontStore>,
    ) -> Result<TextLayout> {
        let attributed = block.to_attributed_string()?;
        self.layout(&attributed, container, font_store)
    }

    fn resolve_runs(
        &self,
        attributed: &AttributedString,
        font_store: Option<&FontStore>,
    ) -> Result<Vec<TextRun>> {
        let mut resolved = Vec::new();

        for run in attributed.canonical_runs()? {
            match self.config.script_itemization {
                ScriptItemization::None => {
                    resolved.push(self.build_text_run(
                        run.range,
                        run.text,
                        run.attributes,
                        font_store,
                    )?);
                }
                ScriptItemization::Heuristic => {
                    for item in self.itemize_run(run.range, &run.text, &run.attributes) {
                        resolved.push(self.build_text_run(
                            item.range,
                            item.text,
                            run.attributes.clone(),
                            font_store,
                        )?);
                    }
                }
            }
        }

        Ok(resolved)
    }

    fn build_text_run(
        &self,
        range: TextRange,
        text: String,
        attributes: TextAttributes,
        font_store: Option<&FontStore>,
    ) -> Result<TextRun> {
        let requested_font = attributes.font().cloned();
        let (resolved_font, font_source) = self.resolve_font(attributes.font(), font_store)?;
        let direction = resolve_direction(attributes.direction(), self.config.bidi, &text);
        let script = detect_script_in_text(&text);
        let decorations = match self.config.text_decoration_mode {
            TextDecorationMode::Preserve => attributes.decorations.clone(),
            TextDecorationMode::Suppress => Vec::new(),
        };

        Ok(TextRun {
            range,
            text,
            requested_font,
            resolved_font,
            font_source,
            font_size: attributes.font_size(),
            letter_spacing: attributes.letter_spacing(),
            language: attributes.language().map(ToOwned::to_owned),
            direction,
            script,
            decorations,
        })
    }

    fn resolve_font(
        &self,
        requested: Option<&FontDescriptor>,
        font_store: Option<&FontStore>,
    ) -> Result<(FontDescriptor, Option<FontSource>)> {
        let requested = requested
            .cloned()
            .unwrap_or_else(|| self.config.default_font.clone());

        let Some(font_store) = font_store else {
            return Ok((requested, None));
        };

        if let Ok(registered) = font_store.get_font(&requested) {
            return Ok((
                registered.descriptor().clone(),
                Some(registered.source().clone()),
            ));
        }

        let mut fallbacks = Vec::new();
        match &self.config.font_substitution {
            FontSubstitution::Disabled => {}
            FontSubstitution::FallbackFamilies(descriptors) => {
                fallbacks.extend(
                    descriptors
                        .iter()
                        .cloned()
                        .map(|descriptor| harmonize_descriptor(descriptor, &requested)),
                );
            }
            FontSubstitution::BestEffort => {
                fallbacks.push(harmonize_descriptor(
                    self.config.default_font.clone(),
                    &requested,
                ));
                fallbacks.push(harmonize_descriptor(
                    FontDescriptor::new(StandardFont::Helvetica.family_name()),
                    &requested,
                ));
                fallbacks.push(harmonize_descriptor(
                    FontDescriptor::new(StandardFont::TimesRoman.family_name()),
                    &requested,
                ));
                fallbacks.push(harmonize_descriptor(
                    FontDescriptor::new(StandardFont::Courier.family_name()),
                    &requested,
                ));
            }
        }

        for fallback in fallbacks {
            if let Ok(registered) = font_store.get_font(&fallback) {
                return Ok((
                    registered.descriptor().clone(),
                    Some(registered.source().clone()),
                ));
            }
        }

        Err(Error::UnresolvedFont {
            family: requested.family().to_string(),
        })
    }

    fn itemize_run(
        &self,
        range: TextRange,
        text: &str,
        attributes: &TextAttributes,
    ) -> Vec<ItemizedRun> {
        if text.is_empty() {
            return Vec::new();
        }

        let explicit_direction = attributes.direction();
        let mut items = Vec::new();
        let mut chunk_start = 0;
        let mut current_script = Script::Common;
        let mut current_direction =
            explicit_direction.unwrap_or_else(|| resolve_direction(None, self.config.bidi, text));

        for (offset, character) in text.char_indices() {
            let script = detect_script(character);
            let direction = explicit_direction.unwrap_or_else(|| {
                direction_for_char(character)
                    .unwrap_or_else(|| resolve_direction(None, self.config.bidi, text))
            });

            let script_changed = current_script != Script::Common
                && script != Script::Common
                && script != current_script;
            let direction_changed = direction != current_direction
                && direction_for_char(character).is_some()
                && has_strong_direction(&text[chunk_start..offset]);

            if offset > chunk_start && (script_changed || direction_changed) {
                let item_text = text[chunk_start..offset].to_string();
                items.push(ItemizedRun {
                    range: TextRange::new(range.start() + chunk_start, range.start() + offset),
                    text: item_text,
                });
                chunk_start = offset;
            }

            if script != Script::Common {
                current_script = script;
            }
            current_direction = direction;
        }

        items.push(ItemizedRun {
            range: TextRange::new(range.start() + chunk_start, range.end()),
            text: text[chunk_start..].to_string(),
        });

        items
    }

    fn layout_runs(
        &self,
        runs: Vec<TextRun>,
        container: TextContainer,
        font_store: Option<&FontStore>,
    ) -> Result<TextLayout> {
        let mut lines = Vec::new();
        let mut pending = Vec::<PendingFragment>::new();
        let mut current_width = 0.0_f32;
        let mut current_y = container.rect().y.value();
        let bottom = container.rect().bottom().value();
        let mut overflowed = false;

        'layout: for run in &runs {
            let tokens = tokenize_run(run, self.config.line_breaking);
            for token in tokens {
                if token.is_newline {
                    if !pending.is_empty() {
                        let line = finalize_line(
                            &pending,
                            container.rect(),
                            current_y,
                            self.config.justification,
                            false,
                        );
                        if line.rect().bottom().value() > bottom {
                            overflowed = true;
                            break 'layout;
                        }
                        current_y = line.rect().bottom().value();
                        lines.push(line);
                        pending.clear();
                        current_width = 0.0;
                    }
                    continue;
                }

                let available_width = container.rect().width.value() - current_width;
                if !pending.is_empty() && token.width.value() > available_width {
                    let line = finalize_line(
                        &pending,
                        container.rect(),
                        current_y,
                        self.config.justification,
                        false,
                    );
                    if line.rect().bottom().value() > bottom {
                        overflowed = true;
                        break 'layout;
                    }
                    current_y = line.rect().bottom().value();
                    lines.push(line);
                    pending.clear();
                    current_width = 0.0;
                }

                let available_width = container.rect().width.value() - current_width;
                if token.width.value() > available_width
                    && !token.is_whitespace
                    && let Some((head, tail)) =
                        self.split_token_to_fit(run, &token, available_width, font_store)
                {
                    pending.push(head);
                    let line = finalize_line(
                        &pending,
                        container.rect(),
                        current_y,
                        self.config.justification,
                        false,
                    );
                    if line.rect().bottom().value() > bottom {
                        overflowed = true;
                        break 'layout;
                    }
                    current_y = line.rect().bottom().value();
                    lines.push(line);
                    pending.clear();
                    current_width = 0.0;

                    if let Some(tail) = tail {
                        let remainder = vec![tail];
                        for fragment in remainder {
                            if fragment.width.value() > container.rect().width.value()
                                && !fragment.is_whitespace
                            {
                                pending.push(fragment);
                                let line = finalize_line(
                                    &pending,
                                    container.rect(),
                                    current_y,
                                    self.config.justification,
                                    false,
                                );
                                if line.rect().bottom().value() > bottom {
                                    overflowed = true;
                                    break 'layout;
                                }
                                current_y = line.rect().bottom().value();
                                lines.push(line);
                                pending.clear();
                                current_width = 0.0;
                            } else {
                                current_width += fragment.width.value();
                                pending.push(fragment);
                            }
                        }
                    }
                    continue;
                }

                current_width += token.width.value();
                pending.push(PendingFragment::from_token(run, token));
            }
        }

        if !overflowed && !pending.is_empty() {
            let line = finalize_line(
                &pending,
                container.rect(),
                current_y,
                self.config.justification,
                true,
            );
            if line.rect().bottom().value() <= bottom {
                lines.push(line);
            } else {
                overflowed = true;
            }
        }

        if let Some(max_lines) = container.max_lines()
            && lines.len() > max_lines
        {
            lines.truncate(max_lines);
            overflowed = true;
        }

        let max_width = lines
            .iter()
            .map(|line| line.rect().width.value())
            .fold(0.0_f32, f32::max);
        let total_height = lines
            .last()
            .map(|line| line.rect().bottom().value() - container.rect().y.value())
            .unwrap_or(0.0);

        Ok(TextLayout {
            lines,
            runs,
            bounds: TextRect::from_values(
                container.rect().x.value(),
                container.rect().y.value(),
                max_width,
                total_height,
            ),
            overflowed,
        })
    }

    fn split_token_to_fit(
        &self,
        run: &TextRun,
        token: &Token,
        available_width: f32,
        font_store: Option<&FontStore>,
    ) -> Option<(PendingFragment, Option<PendingFragment>)> {
        if available_width <= 0.0 {
            return None;
        }

        if let Some((head, tail)) =
            self.try_hyphenate_token(run, token, available_width, font_store)
        {
            return Some((head, Some(tail)));
        }

        if matches!(
            self.config.line_breaking,
            LineBreaking::CharacterBoundary | LineBreaking::WordBoundaryOrCharacter
        ) {
            return split_token_by_characters(run, token, available_width);
        }

        None
    }

    fn try_hyphenate_token(
        &self,
        run: &TextRun,
        token: &Token,
        available_width: f32,
        font_store: Option<&FontStore>,
    ) -> Option<(PendingFragment, PendingFragment)> {
        let WordHyphenation::Enabled {
            min_word_chars,
            marker,
        } = self.config.word_hyphenation
        else {
            return None;
        };

        let store = font_store?;
        if token.text.chars().count() < min_word_chars {
            return None;
        }

        let parts = store.hyphenate(&token.text);
        if parts.len() <= 1 {
            return None;
        }

        let mut byte_count = 0;
        let mut head_text = String::new();
        let mut best_split = None;
        for (index, part) in parts.iter().enumerate().take(parts.len().saturating_sub(1)) {
            head_text.push_str(part);
            byte_count += part.len();
            let candidate = format!("{head_text}{marker}");
            let width = measure_text(&candidate, run.font_size, run.letter_spacing);
            if width <= available_width {
                best_split = Some((index + 1, byte_count, candidate, width));
            } else {
                break;
            }
        }

        let (split_index, byte_count, candidate, width) = best_split?;
        let remainder = parts[split_index..].join("");
        let head = PendingFragment {
            range: TextRange::new(token.range.start(), token.range.start() + byte_count),
            text: candidate,
            width: Pt::new(width),
            direction: run.direction,
            script: run.script,
            font: run.resolved_font.clone(),
            font_size: run.font_size,
            decorations: run.decorations.clone(),
            inserted_hyphen: true,
            is_whitespace: false,
        };
        let tail = PendingFragment {
            range: TextRange::new(token.range.start() + byte_count, token.range.end()),
            width: Pt::new(measure_text(&remainder, run.font_size, run.letter_spacing)),
            text: remainder,
            direction: run.direction,
            script: run.script,
            font: run.resolved_font.clone(),
            font_size: run.font_size,
            decorations: run.decorations.clone(),
            inserted_hyphen: false,
            is_whitespace: false,
        };
        Some((head, tail))
    }
}

#[derive(Clone, Debug)]
struct CanonicalRun {
    range: TextRange,
    text: String,
    attributes: TextAttributes,
}

#[derive(Clone, Debug)]
struct ItemizedRun {
    range: TextRange,
    text: String,
}

#[derive(Clone, Debug)]
struct Token {
    range: TextRange,
    text: String,
    width: Pt,
    is_whitespace: bool,
    is_newline: bool,
}

#[derive(Clone, Debug)]
struct PendingFragment {
    range: TextRange,
    text: String,
    width: Pt,
    direction: TextDirection,
    script: Script,
    font: FontDescriptor,
    font_size: Pt,
    decorations: Vec<TextDecoration>,
    inserted_hyphen: bool,
    is_whitespace: bool,
}

impl PendingFragment {
    fn from_token(run: &TextRun, token: Token) -> Self {
        Self {
            range: token.range,
            text: token.text,
            width: token.width,
            direction: run.direction,
            script: run.script,
            font: run.resolved_font.clone(),
            font_size: run.font_size,
            decorations: run.decorations.clone(),
            inserted_hyphen: false,
            is_whitespace: token.is_whitespace,
        }
    }
}

fn validate_font_size(font_size: Pt) -> Result<()> {
    if font_size.value() <= 0.0 {
        Err(Error::InvalidFontSize {
            size: font_size.value(),
        })
    } else {
        Ok(())
    }
}

fn harmonize_descriptor(descriptor: FontDescriptor, requested: &FontDescriptor) -> FontDescriptor {
    descriptor
        .with_style(requested.font_style())
        .with_weight(requested.font_weight())
}

fn tokenize_run(run: &TextRun, strategy: LineBreaking) -> Vec<Token> {
    match strategy {
        LineBreaking::CharacterBoundary => run
            .text
            .char_indices()
            .map(|(offset, character)| {
                let end = offset + character.len_utf8();
                Token {
                    range: TextRange::new(run.range.start() + offset, run.range.start() + end),
                    text: character.to_string(),
                    width: Pt::new(measure_text(
                        &character.to_string(),
                        run.font_size,
                        run.letter_spacing,
                    )),
                    is_whitespace: character.is_whitespace() && character != '\n',
                    is_newline: character == '\n',
                }
            })
            .collect(),
        LineBreaking::WordBoundary | LineBreaking::WordBoundaryOrCharacter => {
            let mut tokens = Vec::new();
            let mut start = None;
            let mut current_kind = None::<TokenKind>;

            for (offset, character) in run.text.char_indices() {
                let kind = if character == '\n' {
                    TokenKind::Newline
                } else if character.is_whitespace() {
                    TokenKind::Whitespace
                } else {
                    TokenKind::Word
                };

                if current_kind.is_none() {
                    start = Some(offset);
                    current_kind = Some(kind);
                    continue;
                }

                if current_kind != Some(kind) || kind == TokenKind::Newline {
                    if let (Some(token_start), Some(token_kind)) = (start, current_kind) {
                        push_token(&mut tokens, run, token_start, offset, token_kind);
                    }
                    start = Some(offset);
                    current_kind = Some(kind);
                }

                if kind == TokenKind::Newline {
                    if let Some(token_start) = start {
                        push_token(
                            &mut tokens,
                            run,
                            token_start,
                            offset + character.len_utf8(),
                            TokenKind::Newline,
                        );
                    }
                    start = None;
                    current_kind = None;
                }
            }

            if let (Some(token_start), Some(token_kind)) = (start, current_kind) {
                push_token(&mut tokens, run, token_start, run.text.len(), token_kind);
            }

            tokens
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TokenKind {
    Word,
    Whitespace,
    Newline,
}

fn push_token(tokens: &mut Vec<Token>, run: &TextRun, start: usize, end: usize, kind: TokenKind) {
    let text = run.text[start..end].to_string();
    tokens.push(Token {
        range: TextRange::new(run.range.start() + start, run.range.start() + end),
        width: Pt::new(measure_text(&text, run.font_size, run.letter_spacing)),
        is_whitespace: kind == TokenKind::Whitespace,
        is_newline: kind == TokenKind::Newline,
        text,
    });
}

fn split_token_by_characters(
    run: &TextRun,
    token: &Token,
    available_width: f32,
) -> Option<(PendingFragment, Option<PendingFragment>)> {
    let mut last_fit = None;

    for (offset, _) in token.text.char_indices().skip(1) {
        let head = &token.text[..offset];
        let width = measure_text(head, run.font_size, run.letter_spacing);
        match width.partial_cmp(&available_width) {
            Some(Ordering::Greater) => break,
            _ => {
                last_fit = Some((offset, width));
            }
        }
    }

    let (offset, width) = last_fit?;
    let head = PendingFragment {
        range: TextRange::new(token.range.start(), token.range.start() + offset),
        text: token.text[..offset].to_string(),
        width: Pt::new(width),
        direction: run.direction,
        script: run.script,
        font: run.resolved_font.clone(),
        font_size: run.font_size,
        decorations: run.decorations.clone(),
        inserted_hyphen: false,
        is_whitespace: false,
    };
    let tail_text = token.text[offset..].to_string();
    let tail = if tail_text.is_empty() {
        None
    } else {
        Some(PendingFragment {
            range: TextRange::new(token.range.start() + offset, token.range.end()),
            width: Pt::new(measure_text(&tail_text, run.font_size, run.letter_spacing)),
            text: tail_text,
            direction: run.direction,
            script: run.script,
            font: run.resolved_font.clone(),
            font_size: run.font_size,
            decorations: run.decorations.clone(),
            inserted_hyphen: false,
            is_whitespace: false,
        })
    };

    Some((head, tail))
}

fn finalize_line(
    pending: &[PendingFragment],
    container: TextRect,
    line_y: f32,
    justification: Justification,
    is_last_line: bool,
) -> LineFragment {
    let natural_width = pending
        .iter()
        .map(|fragment| fragment.width.value())
        .sum::<f32>();
    let max_font_size = pending
        .iter()
        .map(|fragment| fragment.font_size.value())
        .fold(0.0_f32, f32::max);
    let line_height = if max_font_size > 0.0 {
        max_font_size * 1.2
    } else {
        0.0
    };
    let baseline = line_y + (max_font_size * 0.8);
    let direction = pending
        .iter()
        .find(|fragment| !fragment.is_whitespace)
        .map(|fragment| fragment.direction)
        .unwrap_or(TextDirection::Ltr);
    let available_width = container.width.value();
    let whitespace_slots = pending
        .iter()
        .filter(|fragment| fragment.is_whitespace)
        .count();
    let justify_fully =
        justification == Justification::Full && !is_last_line && whitespace_slots > 0;
    let extra_per_whitespace = if justify_fully && available_width > natural_width {
        (available_width - natural_width) / whitespace_slots as f32
    } else {
        0.0
    };
    let line_width = if justify_fully {
        available_width
    } else {
        natural_width.min(available_width.max(natural_width))
    };
    let slack = (available_width - natural_width).max(0.0);

    let mut cursor = match (direction, justification) {
        (TextDirection::Ltr, Justification::End) => container.x.value() + slack,
        (TextDirection::Ltr, Justification::Center) => container.x.value() + (slack / 2.0),
        (TextDirection::Rtl, Justification::End) => container.right().value() - slack,
        (TextDirection::Rtl, Justification::Center) => container.right().value() - (slack / 2.0),
        (TextDirection::Rtl, _) => container.right().value(),
        _ => container.x.value(),
    };

    let mut fragments = Vec::with_capacity(pending.len());
    for fragment in pending {
        let extra_width = if justify_fully && fragment.is_whitespace {
            extra_per_whitespace
        } else {
            0.0
        };
        let fragment_width = fragment.width.value() + extra_width;
        let x = match direction {
            TextDirection::Ltr => {
                let x = cursor;
                cursor += fragment_width;
                x
            }
            TextDirection::Rtl => {
                cursor -= fragment_width;
                cursor
            }
        };

        fragments.push(TextFragment {
            range: fragment.range,
            text: fragment.text.clone(),
            rect: TextRect::from_values(x, line_y, fragment_width, line_height),
            baseline: Pt::new(baseline),
            direction: fragment.direction,
            script: fragment.script,
            font: fragment.font.clone(),
            font_size: fragment.font_size,
            decorations: fragment.decorations.clone(),
            inserted_hyphen: fragment.inserted_hyphen,
            whitespace: fragment.is_whitespace,
        });
    }

    LineFragment {
        rect: TextRect::from_values(container.x.value(), line_y, line_width, line_height),
        baseline: Pt::new(baseline),
        direction,
        justification,
        fragments,
    }
}

fn measure_text(text: &str, font_size: Pt, letter_spacing: Pt) -> f32 {
    let mut width = 0.0_f32;
    let mut characters = 0_usize;

    for character in text.chars() {
        width += glyph_advance(character, font_size);
        characters += 1;
    }

    if characters > 1 {
        width += letter_spacing.value() * (characters.saturating_sub(1) as f32);
    }

    width
}

fn glyph_advance(character: char, font_size: Pt) -> f32 {
    let multiplier = if character == ' ' {
        0.33
    } else if character == '\t' {
        1.32
    } else if matches!(detect_script(character), Script::Han) {
        1.0
    } else if matches!(detect_script(character), Script::Arabic | Script::Hebrew) {
        0.68
    } else if character.is_ascii_uppercase() {
        0.62
    } else if character.is_ascii_punctuation() {
        0.35
    } else if character.is_ascii_digit() {
        0.55
    } else if character.is_alphabetic() {
        0.56
    } else {
        0.5
    };

    font_size.value() * multiplier
}

fn resolve_direction(explicit: Option<TextDirection>, bidi: BidiMode, text: &str) -> TextDirection {
    if let Some(direction) = explicit {
        return direction;
    }

    match bidi {
        BidiMode::LeftToRight => TextDirection::Ltr,
        BidiMode::RightToLeft => TextDirection::Rtl,
        BidiMode::Auto => text
            .chars()
            .find_map(direction_for_char)
            .unwrap_or(TextDirection::Ltr),
    }
}

fn has_strong_direction(text: &str) -> bool {
    text.chars()
        .any(|character| direction_for_char(character).is_some())
}

fn direction_for_char(character: char) -> Option<TextDirection> {
    match detect_script(character) {
        Script::Arabic | Script::Hebrew => Some(TextDirection::Rtl),
        Script::Latin | Script::Cyrillic | Script::Han => Some(TextDirection::Ltr),
        Script::Common | Script::Unknown => None,
    }
}

fn detect_script_in_text(text: &str) -> Script {
    text.chars()
        .map(detect_script)
        .find(|script| *script != Script::Common)
        .unwrap_or(Script::Common)
}

fn detect_script(character: char) -> Script {
    let code = character as u32;

    if character.is_ascii_alphabetic() {
        Script::Latin
    } else if character.is_whitespace()
        || character.is_ascii_punctuation()
        || character.is_ascii_digit()
    {
        Script::Common
    } else if (0x0600..=0x06ff).contains(&code)
        || (0x0750..=0x077f).contains(&code)
        || (0x08a0..=0x08ff).contains(&code)
    {
        Script::Arabic
    } else if (0x0590..=0x05ff).contains(&code) {
        Script::Hebrew
    } else if (0x0400..=0x052f).contains(&code) {
        Script::Cyrillic
    } else if (0x4e00..=0x9fff).contains(&code)
        || (0x3400..=0x4dbf).contains(&code)
        || (0xf900..=0xfaff).contains(&code)
    {
        Script::Han
    } else if character.is_alphabetic() {
        Script::Latin
    } else {
        Script::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use graphitepdf_font::FontStore;

    #[test]
    fn converts_text_block_into_attributed_string_runs() {
        let block = TextBlock::new([
            TextSpan::new("Hello")
                .expect("valid span")
                .with_font(FontDescriptor::new("Helvetica"))
                .with_font_size(Pt::new(14.0))
                .expect("valid font size"),
            TextSpan::new(" world")
                .expect("valid span")
                .with_font_size(Pt::new(12.0))
                .expect("valid font size"),
        ]);

        let attributed = block
            .to_attributed_string()
            .expect("block should convert to attributed text");

        assert_eq!(attributed.text(), "Hello world");
        assert_eq!(attributed.runs().len(), 2);
        assert_eq!(attributed.runs()[0].range(), TextRange::new(0, 5));
        assert_eq!(
            attributed.runs()[0]
                .attributes()
                .font()
                .map(|font| font.family().to_string()),
            Some(String::from("Helvetica"))
        );
    }

    #[test]
    fn itemizes_scripts_and_substitutes_missing_fonts() {
        let store = FontStore::new();
        let attributes = TextAttributes::default()
            .with_font(FontDescriptor::new("MissingFamily"))
            .with_font_size(Pt::new(12.0))
            .expect("font size is valid");
        let attributed = AttributedString::new("Hello مرحبا")
            .expect("text should be valid")
            .with_default_attributes(attributes)
            .expect("default attributes should be valid");
        let container = TextContainer::new(TextRect::from_values(0.0, 0.0, 200.0, 80.0))
            .expect("container should be valid");
        let engine = TextEngine::new(TextEngineConfig {
            font_substitution: FontSubstitution::FallbackFamilies(vec![FontDescriptor::new(
                "Helvetica",
            )]),
            ..TextEngineConfig::default()
        });

        let layout = engine
            .layout(&attributed, &container, Some(&store))
            .expect("layout should succeed with fallback fonts");

        assert!(layout.runs().len() >= 2);
        assert_eq!(layout.runs()[0].script(), Script::Latin);
        assert!(
            layout
                .runs()
                .iter()
                .any(|run| run.script() == Script::Arabic)
        );
        assert!(
            layout
                .runs()
                .iter()
                .all(|run| run.resolved_font().family() == "Helvetica")
        );
    }

    #[test]
    fn hyphenates_long_words_in_narrow_containers() {
        let mut store = FontStore::new();
        store.register_hyphenation_callback(|word| {
            if word == "graphitepdf" {
                vec!["graph".into(), "ite".into(), "pdf".into()]
            } else {
                vec![word.to_string()]
            }
        });

        let attributed = AttributedString::new("graphitepdf")
            .expect("text should be valid")
            .with_default_attributes(
                TextAttributes::default()
                    .with_font(FontDescriptor::new("Helvetica"))
                    .with_font_size(Pt::new(12.0))
                    .expect("font size is valid"),
            )
            .expect("default attributes should be valid");
        let container = TextContainer::new(TextRect::from_values(0.0, 0.0, 40.0, 100.0))
            .expect("container should be valid");
        let engine = TextEngine::new(TextEngineConfig {
            word_hyphenation: WordHyphenation::Enabled {
                min_word_chars: 6,
                marker: '-',
            },
            line_breaking: LineBreaking::WordBoundaryOrCharacter,
            ..TextEngineConfig::default()
        });

        let layout = engine
            .layout(&attributed, &container, Some(&store))
            .expect("hyphenated layout should succeed");

        assert!(layout.lines().len() >= 2);
        assert_eq!(layout.lines()[0].fragments()[0].text(), "graph-");
        assert!(layout.lines()[0].fragments()[0].inserted_hyphen());
    }

    #[test]
    fn fully_justifies_intermediate_lines_and_preserves_decorations() {
        let store = FontStore::new();
        let attributed = AttributedString::new("rust text layout")
            .expect("text should be valid")
            .with_default_attributes(
                TextAttributes::default()
                    .with_font(FontDescriptor::new("Helvetica"))
                    .with_font_size(Pt::new(12.0))
                    .expect("font size is valid")
                    .with_decoration(TextDecoration::new(TextDecorationKind::Underline)),
            )
            .expect("default attributes should be valid");
        let container = TextContainer::new(TextRect::from_values(0.0, 0.0, 75.0, 100.0))
            .expect("container should be valid");
        let engine = TextEngine::new(TextEngineConfig {
            justification: Justification::Full,
            ..TextEngineConfig::default()
        });

        let layout = engine
            .layout(&attributed, &container, Some(&store))
            .expect("layout should succeed");

        assert!(layout.lines().len() >= 2);
        assert_eq!(layout.lines()[0].rect().width.value(), 75.0);
        assert!(
            layout.lines()[0]
                .fragments()
                .iter()
                .all(|fragment| !fragment.decorations().is_empty())
        );
    }

    #[test]
    fn rtl_layout_places_fragments_from_right_to_left() {
        let store = FontStore::new();
        let attributed = AttributedString::new("مرحبا بكم")
            .expect("text should be valid")
            .with_default_attributes(
                TextAttributes::default()
                    .with_font(FontDescriptor::new("Helvetica"))
                    .with_font_size(Pt::new(12.0))
                    .expect("font size is valid"),
            )
            .expect("default attributes should be valid");
        let container = TextContainer::new(TextRect::from_values(0.0, 0.0, 120.0, 60.0))
            .expect("container should be valid");
        let engine = TextEngine::new(TextEngineConfig {
            bidi: BidiMode::RightToLeft,
            ..TextEngineConfig::default()
        });

        let layout = engine
            .layout(&attributed, &container, Some(&store))
            .expect("layout should succeed");
        let first_line = &layout.lines()[0];

        assert_eq!(first_line.direction(), TextDirection::Rtl);
        assert!(
            first_line.fragments()[0].rect().x.value() > first_line.fragments()[2].rect().x.value()
        );
    }
}
