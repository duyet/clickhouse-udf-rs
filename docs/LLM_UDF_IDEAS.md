# LLM-Based UDF Ideas for ClickHouse

## Current State
The project already has `tiktoken` for GPT tokenization. Here are additional LLM-based UDF ideas that would be valuable for ClickHouse users.

## 1. **Text Embedding UDFs**
Generate vector embeddings for semantic search and similarity matching.

### Potential Implementations:
- `text-embedding-ada-002` using OpenAI API
- `all-MiniLM-L6-v2` using local sentence-transformers (Rust + Python bridge)
- `bge-small-en-v1.5` using local models

### Use Cases:
- Semantic search in text data
- Document similarity matching
- Clustering text by semantic meaning
- RAG (Retrieval Augmented Generation) applications

```sql
SELECT textEmbedding('This is a sample document');
-- Returns: [0.1, -0.2, 0.3, ...]

SELECT cosine_similarity(textEmbedding(doc1), textEmbedding(doc2)) as similarity
FROM documents;
```

## 2. **Text Classification UDFs**
Classify text into categories using LLMs.

### Categories:
- Sentiment analysis (positive/negative/neutral)
- Topic classification
- Intent detection
- Language detection
- Spam detection

```sql
SELECT classifySentiment('I love this product!');
-- Returns: "positive"

SELECT classifyTopic('The stock market crashed today due to inflation concerns');
-- Returns: "finance"
```

## 3. **Text Summarization UDFs**
Generate summaries of long text content.

### Variants:
- `summarizeText` - General summarization
- `summarizeBulletPoints` - Bullet point summary
- `summarizeWithMaxTokens` - Summary with token limit

```sql
SELECT summarizeText(article_content) as summary
FROM news_articles
WHERE length > 1000;
```

## 4. **Named Entity Recognition (NER) UDFs**
Extract named entities from text.

### Entity Types:
- Person names
- Organizations
- Locations
- Dates/times
- Phone numbers
- Email addresses
- URLs

```sql
SELECT extractEntities('John Doe works at Google in California');
-- Returns: JSON with entities: [{"text": "John Doe", "type": "PERSON"}, ...]
```

## 5. **Text Cleaning & Normalization UDFs**
Prepare text for LLM processing.

### Functions:
- `normalizeText` - Lowercase, remove extra whitespace, standardize punctuation
- `removePersonalInfo` - Redact emails, phone numbers, SSNs
- `expandContractions` - Convert "don't" to "do not"
- `removeStopwords` - Remove common words (the, is, at, etc.)

```sql
SELECT normalizeText('  Hello    World!  ')
-- Returns: "hello world"

SELECT removePersonalInfo('Contact me at john@example.com or 555-1234')
-- Returns: "Contact me at [EMAIL] or [PHONE]"
```

## 6. **Question Answering UDFs**
Extract answers from text given a question.

```sql
SELECT extractAnswer(article_text, 'What is the main cause?')
-- Returns: "The main cause is..."
```

## 7. **Keyword Extraction UDFs**
Extract important keywords/keyphrases from text.

```sql
SELECT extractKeywords('Machine learning is a subset of artificial intelligence...')
-- Returns: ["machine learning", "artificial intelligence", ...]
```

## 8. **Text Similarity UDFs**
Calculate similarity between two texts without full embeddings.

### Similarity Metrics:
- Jaccard similarity
- Cosine similarity (with embeddings)
- Levenshtein distance
- Semantic similarity (using sentence-transformers)

```sql
SELECT textSimilarity('Hello world', 'Hello there', 'cosine')
-- Returns: 0.85
```

## 9. **Language Detection UDFs**
Detect the language of input text.

```sql
SELECT detectLanguage('Bonjour le monde')
-- Returns: "french"

SELECT detectLanguage('Hola mundo')
-- Returns: "spanish"
```

## 10. **PII Detection UDFs**
Detect and optionally redact personally identifiable information.

### PII Types:
- Email addresses
- Phone numbers
- Social Security Numbers
- Credit card numbers
- IP addresses
- Addresses

```sql
SELECT detectPII('Contact me at john@example.com or 555-1234')
-- Returns: {"email": ["john@example.com"], "phone": ["555-1234"]}

SELECT redactPII('Contact me at john@example.com', 'email')
-- Returns: "Contact me at [REDACTED]"
```

## 11. **Batch Processing UDFs**
Process multiple texts efficiently (for array inputs).

```sql
SELECT batchEmbed(['text1', 'text2', 'text3'])
-- Returns: [[0.1, 0.2], [0.3, 0.4], [0.5, 0.6]]
```

## 12. **LLM API Wrapper UDFs**
Direct integration with LLM APIs for flexible processing.

### Supported APIs:
- OpenAI (GPT-4, GPT-3.5-turbo)
- Anthropic (Claude)
- Cohere
- Local models (Ollama, llama.cpp)

```sql
SELECT callLLM('gpt-4', 'Summarize this: ' || text)
-- Returns: LLM response

SELECT callLLM('claude-3', 'Extract sentiment: ' || review)
-- Returns: "positive"
```

## Implementation Considerations

### Performance:
1. **Caching** - Cache embeddings and LLM responses
2. **Batching** - Process multiple texts in single API calls
3. **Async Processing** - Use async Rust for API calls
4. **Connection Pooling** - Reuse HTTP connections

### Cost Management:
1. **Token Counting** - Use tiktoken to pre-count tokens
2. **Rate Limiting** - Implement per-UDF rate limits
3. **Fallback** - Graceful degradation on API failures
4. **Local Models** - Support local alternatives for cost savings

### Security:
1. **API Key Management** - Secure storage of API keys
2. **Input Validation** - Prevent injection attacks
3. **Rate Limiting** - Prevent abuse
4. **Logging** - Track usage for cost monitoring

### Configuration:
```toml
# ClickHouse UDF config
<function>
    <name>textEmbedding</name>
    <type>executable_pool</type>
    <command>text-embedding</command>
    <environment>
        <OPENAI_API_KEY>/path/to/key</OPENAI_API_KEY>
        <EMBEDDING_MODEL>text-embedding-3-small</EMBEDDING_MODEL>
        <CACHE_SIZE>1000</CACHE_SIZE>
    </environment>
</function>
```

## Priority Ranking

### High Priority (Most Value):
1. **Text Embeddings** - Critical for semantic search
2. **Sentiment Classification** - Common analytics need
3. **PII Detection/Redaction** - Privacy compliance
4. **Language Detection** - Multi-language support

### Medium Priority:
5. **Text Summarization** - Content analysis
6. **NER** - Information extraction
7. **Text Normalization** - Data quality

### Lower Priority (Niche):
8. **Question Answering** - Specific use cases
9. **Keyword Extraction** - Can be done with other methods
10. **Direct LLM API** - Too generic, may be better as a separate tool

## Next Steps

1. Start with **text embeddings** using a local model (sentence-transformers)
2. Add **sentiment classification** using a small, fast model
3. Implement **PII detection** using regex + ML hybrid approach
4. Consider OpenAI API integration for cloud-based embeddings

## Technical Architecture

```rust
// Example structure for embedding UDF
use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_transformers::models::bert::{BertModel, Config};

pub fn text_embedding(input: &str) -> Option<String> {
    // Load model (cached)
    // Tokenize input
    // Run inference
    // Return embedding as JSON array
    Some(embedding.to_json())
}
```

## Rust ML Libraries to Consider

1. **candle** - Hugging Face's Rust ML framework
2. **burn** - Deep learning framework in pure Rust
3. **tract** - Neural network inference
4. **rust-bert** - BERT models in Rust
5. **sqruff** - Local sentence embeddings

## References

- [candle: ML framework in Rust](https://github.com/huggingface/candle)
- [rust-bert: BERT in Rust](https://github.com/guillaume-be/rust-bert)
- [ClickHouse External UDFs](https://clickhouse.com/docs/en/sql-reference/functions/external-user-defined-functions)

---

## Additional LLM-Based UDF Ideas

### 13. **Text Deduplication UDFs**

Detect near-duplicate texts using semantic similarity or hashing.

```sql
SELECT textDedupHash('This is a sample document')
-- Returns: MinHash or SimHash for similarity comparison

SELECT isNearDuplicate('text1', 'text2', 0.85)
-- Returns: 1 if similarity > 0.85
```

**Use Cases:**
- Deduplicating customer feedback
- Removing duplicate social media posts
- Identifying similar documents

**Implementation:**
- MinHash or SimHash for fast approximation
- Cosine similarity with embeddings for accuracy

### 14. **Text Complexity Scoring UDFs**

Calculate readability scores and complexity metrics.

```sql
SELECT textComplexity('The quick brown fox...')
-- Returns: JSON with scores (flesch, fog_level, etc.)

SELECT fleschReadingEase(article_content)
SELECT fleschKincaidGrade(article_content)
```

**Metrics:**
- Flesch Reading Ease
- Flesch-Kincaid Grade Level
- Gunning Fog Index
- Sentence length, word length statistics

**Use Cases:**
- Content quality assessment
- Educational content classification
- SEO optimization

### 15. **Email Parsing and Extraction UDFs**

Parse emails and extract structured information.

```sql
SELECT extractEmailParts(raw_email)
-- Returns: JSON with from, to, subject, body, etc.

SELECT extractEmailThread(raw_email)
-- Returns: Thread/conversation ID
```

**Use Cases:**
- Email analytics
- Customer support analysis
- Communication pattern analysis

### 16. **JSON/Structured Data Extraction UDFs**

Extract structured data from unstructured text using LLMs.

```sql
SELECT extractJSON('Name: John, Age: 30, City: NYC')
-- Returns: {"name": "John", "age": 30, "city": "NYC"}

SELECT extractTable('Product: Laptop, Price: $999')
-- Returns: Structured table data
```

**Use Cases:**
- Invoice processing
- Resume parsing
- Form data extraction

### 17. **Topic Modeling UDFs**

Assign topics to text using pre-trained models or LDA.

```sql
SELECT assignTopic('The stock market crashed today...')
-- Returns: "finance" or probability distribution

SELECT topicDistribution(article_text)
-- Returns: {"finance": 0.8, "politics": 0.1, ...}
```

**Use Cases:**
- Content categorization
- News aggregation
- Document organization

### 18. **Text Generation UDFs**

Generate text based on templates or LLM completion.

```sql
SELECT generateSummaryTemplate(title, content, template)
-- Returns: Generated summary using template

SELECT completeText('The quick brown fox...', max_tokens=10)
-- Returns: Text completion
```

**Use Cases:**
- Automated reporting
- Content generation
- Text completion

### 19. **Sentiment Scoring (Fine-grained) UDFs**

More detailed sentiment analysis beyond positive/negative/neutral.

```sql
SELECT sentimentScore('I love this product!')
-- Returns: 0.95 (confidence score)

SELECT sentimentWithEmotions('I am angry about this!')
-- Returns: {"sentiment": "negative", "emotions": {"anger": 0.8, ...}}
```

**Emotions:**
- Joy, sadness, anger, fear, surprise, disgust
- Valence and arousal scores

**Use Cases:**
- Customer feedback analysis
- Social media monitoring
- Brand sentiment tracking

### 20. **Text Translation UDFs**

Translate text between languages using local or API models.

```sql
SELECT translateText('Hello world', 'en', 'es')
-- Returns: "Hola mundo"

SELECT detectAndTranslate('Bonjour le monde', 'en')
-- Returns: "Hello world"
```

**Use Cases:**
- Multi-language support
- Content localization
- Cross-language analytics

### 21. **Time/Date Extraction UDFs**

Extract temporal expressions from text.

```sql
SELECT extractDates('Meeting scheduled for next Tuesday at 3pm')
-- Returns: ["2024-01-09T15:00:00"]

SELECT normalizeDate('in 2 weeks', '2024-01-01')
-- Returns: "2024-01-15"
```

**Use Cases:**
- Scheduling systems
- Event extraction
- Document timeline analysis

### 22. **Currency/Money Extraction UDFs**

Extract and normalize monetary values.

```sql
SELECT extractMoney('Price: $999.99')
-- Returns: {"amount": 999.99, "currency": "USD"}

SELECT normalizeCurrency('€100', 'USD', rate=1.1)
-- Returns: 110.0
```

**Use Cases:**
- Financial document processing
- Price comparison
- E-commerce analytics

### 23. **Text Clustering UDFs**

Group similar texts into clusters.

```sql
SELECT textClusterId(text, num_clusters=10)
-- Returns: Cluster ID (0-9)

SELECT clusterTopics(arrayJoin(texts))
-- Returns: Dominant topics per cluster
```

**Use Cases:**
- Document grouping
- Customer segmentation
- Content organization

### 24. **Semantic Search UDFs**

Find semantically similar texts.

```sql
SELECT findSimilar(query, table.column, limit=5)
-- Returns: Top 5 most similar texts

SELECT semanticSimilarity(text1, text2)
-- Returns: Similarity score (0-1)
```

**Use Cases:**
- Search engines
- Recommendation systems
- Duplicate detection

### 25. **Text Quality Scoring UDFs**

Assess overall text quality.

```sql
SELECT textQualityScore(review_text)
-- Returns: JSON with grammar, spelling, clarity scores

SELECT hasGrammarIssues('This sentence have a error')
-- Returns: 1 (true)
```

**Use Cases:**
- Content moderation
- Quality assurance
- Automated editing

---

## Implementation Roadmap

### Phase 1: Simple Regex/Rule-Based UDFs (Quick Wins)
- **PII Detection** (phone, email, SSN, credit card)
- **Phone Extraction** (already exists, extend)
- **Text Cleaning/Normalization**
- **Text Complexity Scoring**
- **Currency/Money Extraction**

### Phase 2: Local ML Models (Medium Complexity)
- **Sentiment Classification** (use fasttext or similar)
- **Language Detection** (use whatlang-rs or cld)
- **Text Deduplication** (MinHash/SimHash)
- **Keyword Extraction** (TF-IDF, RAKE)

### Phase 3: Embeddings-based UDFs (High Value)
- **Text Embeddings** (sentence-transformers via candle)
- **Semantic Similarity**
- **Semantic Search**
- **Text Clustering**

### Phase 4: API-based UDFs (High Cost, High Value)
- **Text Summarization** (OpenAI/Claude API)
- **NER** (spaCy or OpenAI)
- **Text Generation**
- **Translation** (DeepL or OpenAI)

### Phase 5: Advanced ML Models
- **Topic Modeling** (LDA, BERT-based)
- **Question Answering**
- **Text Quality Scoring**

---

## Quick Start Implementation: PII Detection

Here's a simple implementation that can be done quickly:

```rust
// pii/Cargo.toml
[dependencies]
anyhow = "1.0"
regex = "1.10"
shared = { path = "../shared" }

// pii/src/lib.rs
use regex::Regex;
use std::sync::LazyLock;

static EMAIL_RE: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap()
);

static PHONE_RE: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"\+?\d{1,3}[-.\s]?\(?\d{1,4}\)?[-.\s]?\d{1,4}[-.\s]?\d{1,9}").unwrap()
);

static SSN_RE: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").unwrap()
);

static CREDIT_CARD_RE: LazyLock<Regex> = LazyLock::new(
    || Regex::new(r"\b(?:\d[ -]*?){13,16}\b").unwrap()
);

pub fn detect_pii(s: &str) -> Option<String> {
    let mut pii_found = std::collections::HashMap::new();

    if EMAIL_RE.is_match(s) {
        pii_found.insert("email", true);
    }
    if PHONE_RE.is_match(s) {
        pii_found.insert("phone", true);
    }
    if SSN_RE.is_match(s) {
        pii_found.insert("ssn", true);
    }
    if CREDIT_CARD_RE.is_match(s) {
        pii_found.insert("credit_card", true);
    }

    if pii_found.is_empty() {
        None
    } else {
        Some(serde_json::to_string(&pii_found).unwrap())
    }
}

pub fn redact_pii(s: &str) -> Option<String> {
    let result = EMAIL_RE.replace_all(s, "[EMAIL]");
    let result = PHONE_RE.replace_all(&result, "[PHONE]");
    let result = SSN_RE.replace_all(&result, "[SSN]");
    let result = CREDIT_CARD_RE.replace_all(&result, "[CARD]");
    Some(result.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_pii() {
        let text = "Contact me at john@example.com or 555-123-4567";
        let result = detect_pii(text);
        assert!(result.is_some());
        let pii: serde_json::Value = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(pii["email"], true);
        assert_eq!(pii["phone"], true);
    }

    #[test]
    fn test_redact_pii() {
        let text = "Email: john@example.com, Phone: 555-123-4567";
        let result = redact_pii(text).unwrap();
        assert!(result.contains("[EMAIL]"));
        assert!(result.contains("[PHONE]"));
        assert!(!result.contains("john@example.com"));
    }
}
```

This PII detection UDF:
- Is simple to implement (no ML required)
- Provides immediate value (privacy compliance)
- Can be extended with more patterns
- Works fast (regex-based)
- Can be a building block for more advanced features
