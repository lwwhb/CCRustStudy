#!/bin/bash

# CCRustStudy 项目验证脚本

echo "🔍 CCRustStudy 项目验证"
echo "========================"
echo ""

# 颜色定义
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 计数器
PASS=0
FAIL=0

# 检查函数
check() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $1"
        ((PASS++))
    else
        echo -e "${RED}✗${NC} $1"
        ((FAIL++))
    fi
}

# 1. 检查教程文件
echo "📚 检查教程文件..."
TUTORIAL_COUNT=$(find . -name "TUTORIAL.md" -type f | wc -l | tr -d ' ')
if [ "$TUTORIAL_COUNT" -eq 35 ]; then
    echo -e "${GREEN}✓${NC} 找到 35 个 TUTORIAL.md 文件"
    ((PASS++))
else
    echo -e "${RED}✗${NC} 找到 $TUTORIAL_COUNT 个 TUTORIAL.md 文件（应该是 35 个）"
    ((FAIL++))
fi

# 2. 检查核心文档
echo ""
echo "📖 检查核心文档..."
for doc in README.md CLAUDE.md LEARNING_GUIDE.md INDEX.md QUICK_REFERENCE.md PROJECT_SUMMARY.md; do
    if [ -f "$doc" ]; then
        echo -e "${GREEN}✓${NC} $doc 存在"
        ((PASS++))
    else
        echo -e "${RED}✗${NC} $doc 缺失"
        ((FAIL++))
    fi
done

# 3. 检查阶段目录
echo ""
echo "📁 检查阶段目录..."
for stage in 01-foundation 02-intermediate 03-advanced 04-graphics-foundation 05-web-services 06-3d-renderer 07-ai-gateway; do
    if [ -d "$stage" ]; then
        echo -e "${GREEN}✓${NC} $stage 目录存在"
        ((PASS++))
    else
        echo -e "${RED}✗${NC} $stage 目录缺失"
        ((FAIL++))
    fi
done

# 4. 检查 Cargo 配置
echo ""
echo "🦀 检查 Cargo 配置..."
if [ -f "Cargo.toml" ]; then
    echo -e "${GREEN}✓${NC} Cargo.toml 存在"
    ((PASS++))
else
    echo -e "${RED}✗${NC} Cargo.toml 缺失"
    ((FAIL++))
fi

# 5. 检查 .gitignore
echo ""
echo "🔒 检查 .gitignore..."
if [ -f ".gitignore" ]; then
    echo -e "${GREEN}✓${NC} .gitignore 存在"
    ((PASS++))
else
    echo -e "${RED}✗${NC} .gitignore 缺失"
    ((FAIL++))
fi

# 6. 统计字数（粗略估算）
echo ""
echo "📊 统计教程字数..."
TOTAL_WORDS=$(find . -name "TUTORIAL.md" -type f -exec wc -w {} \; | awk '{sum+=$1} END {print sum}')
echo "   总字数约：$TOTAL_WORDS 字"
if [ "$TOTAL_WORDS" -gt 300000 ]; then
    echo -e "${GREEN}✓${NC} 字数超过 300,000"
    ((PASS++))
else
    echo -e "${YELLOW}⚠${NC} 字数少于预期"
fi

# 7. 检查模块结构
echo ""
echo "🏗️  检查模块结构..."
MODULE_DIRS=$(find . -maxdepth 2 -type d -name "[0-9]*-*" | wc -l | tr -d ' ')
if [ "$MODULE_DIRS" -eq 35 ]; then
    echo -e "${GREEN}✓${NC} 找到 35 个模块目录"
    ((PASS++))
else
    echo -e "${YELLOW}⚠${NC} 找到 $MODULE_DIRS 个模块目录"
fi

# 总结
echo ""
echo "========================"
echo "📊 验证结果"
echo "========================"
echo -e "通过: ${GREEN}$PASS${NC}"
echo -e "失败: ${RED}$FAIL${NC}"
echo ""

if [ $FAIL -eq 0 ]; then
    echo -e "${GREEN}🎉 项目验证通过！所有检查都成功！${NC}"
    exit 0
else
    echo -e "${RED}⚠️  项目验证失败，请检查上述错误。${NC}"
    exit 1
fi
