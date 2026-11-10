---
name: pharmrag
description: Search and retrieve GxP/pharma regulatory documents (NMPA/CDE/FDA/ICH) with exact clause-level citations, AST structure, and PDF page numbers.
tools:
  - name: pharmrag_search
    description: Query the local PharmRAG regulatory knowledge base using Hybrid RRF (Trigram + Dense vector).
---

# PharmRAG Agent Skill

Use `pharmrag` to query authoritative pharmaceutical regulations and GxP guidelines with precise clause AST context (`章 > 节 > 条`) and physical PDF page mapping.

## When to Use

- Looking up CDE, NMPA, FDA, EMA, or ICH guidance documents.
- Verifying CMC quality control standards, release testing requirements, or stability testing guidelines.
- Checking regulatory status (effective vs. draft/trial) before submitting IND/NDA filings or auditing SOPs.
- Cross-referencing technical requirements (e.g., AAV empty/full capsid ratios, CAR-T RCL/RVV testing, residual DNA/host cell protein limits).

## CLI Usage Guidelines

### 1. Standard Query
Query via CLI using `--json` to receive structured, verifiable evidence for downstream agent reasoning:
```bash
pharmrag query "<question or keywords>" --limit 3 --min-score 0.40 --json
```

Or run through standard search:
```bash
pharmrag search "<question or keywords>" --hybrid --limit 3 --min-score 0.40 --json
```

### 2. Common CLI Options
- `--limit <N>`: Return top N clauses (default: 3, recommended: 3-5).
- `--min-score <F>`: RRF cutoff score (default: 0.40). Lower to 0.30 for broad exploratory searches; pass 0.0 to disable.
- `--only-effective`: Strict filter to exclude draft consultation papers (`draft`), retaining only effective and interim trial guidelines.
- `--status <STATUS>`: Filter strictly by lifecycle status (`effective`, `trial`, `draft`, `superseded`).
- `--json`: Output valid JSON for programmatic parsing.

### 3. GxP Compliance & Citation Rules
When generating answers based on PharmRAG results, all AI agents must follow these principles:
1. **Clause-Level Provenance**: Always cite the exact Document Title, Clause hierarchy (e.g., `第三章 > 第十二条`), and `page_num`.
2. **Regulatory Lifecycle Warning**: If `status == "draft"`, explicitly caution the user:
   > ⚠️ **注意**：该依据源自征求意见稿，正式申报与 GMP 合规请核对现行有效版指导原则。
3. **Never Fabricate Values**: If numerical thresholds or parameters are not explicitly present in retrieved clauses, state that the guideline specifies risk-based determination or is silent.
