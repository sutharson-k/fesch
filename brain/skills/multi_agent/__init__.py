# brain/skills/multi_agent/__init__.py
# Multi-agent neural network visualization for complex problem solving

from google.adk.agents import Agent
from brain.model_router import OpenRouterModel
import asyncio
import json

multi_agent_agent = Agent(
    name="multi_agent",
    model=OpenRouterModel(),
    description="Multi-agent debate system for complex problem solving",
    instruction="""
You coordinate multiple AI agents debating to find the best solution.
Each agent has a different perspective:
- Analyst: Breaks down the problem logically
- Critic: Identifies flaws and edge cases  
- Innovator: Proposes creative solutions
- Validator: Tests and verifies solutions

Facilitate the debate and converge on the best answer.
""",
    tools=[],
)


class MultiAgentDebate:
    """Manages multi-agent debate for complex problems."""

    def __init__(self, problem: str):
        self.problem = problem
        self.agents = {
            "analyst": Agent(
                name="analyst",
                model=OpenRouterModel(),
                instruction="Analyze problems logically and systematically.",
            ),
            "critic": Agent(
                name="critic", 
                model=OpenRouterModel(),
                instruction="Critique solutions, find flaws and edge cases.",
            ),
            "innovator": Agent(
                name="innovator",
                model=OpenRouterModel(),
                instruction="Propose creative and innovative solutions.",
            ),
            "validator": Agent(
                name="validator",
                model=OpenRouterModel(),
                instruction="Validate and test proposed solutions.",
            ),
        }
        self.debate_history = []
        self.consensus = None

    async def run_debate(self, rounds: int = 3) -> str:
        """Run a multi-round debate between agents."""
        for round_num in range(rounds):
            round_data = {"round": round_num + 1, "statements": []}

            # Each agent contributes
            for role, agent in self.agents.items():
                statement = await self._get_agent_statement(
                    agent, role, round_num
                )
                round_data["statements"].append({
                    "role": role,
                    "statement": statement,
                })
                self.debate_history.append((role, statement))

            # Check for consensus
            if self._check_consensus():
                break

        return self._synthesize_solution()

    async def _get_agent_statement(
        self, agent: Agent, role: str, round_num: int
    ) -> str:
        """Get a statement from an agent."""
        context = self._build_context(role, round_num)
        # In full implementation, would call agent.run()
        return f"[{role}] Analysis for round {round_num + 1}"

    def _build_context(self, role: str, round_num: int) -> str:
        """Build context for an agent's turn."""
        context = f"Problem: {self.problem}\n\n"
        if self.debate_history:
            context += "Previous statements:\n"
            for r, stmt in self.debate_history[-4:]:
                if r != role:
                    context += f"- {r}: {stmt}\n"
        return context

    def _check_consensus(self) -> bool:
        """Check if agents have reached consensus."""
        if len(self.debate_history) < 8:
            return False
        # Simplified consensus check
        return False

    def _synthesize_solution(self) -> str:
        """Synthesize the final solution from debate."""
        return "Synthesized solution from multi-agent debate"


async def run_debate(problem: str, rounds: int = 3) -> dict:
    """
    Run a multi-agent debate to solve a complex problem.
    
    Args:
        problem: The problem to solve
        rounds: Number of debate rounds
        
    Returns:
        Debate results including all statements and final solution
    """
    debate = MultiAgentDebate(problem)
    solution = await debate.run_debate(rounds)

    return {
        "problem": problem,
        "debate_history": debate.debate_history,
        "solution": solution,
        "rounds_completed": len(set(r for r, _ in debate.debate_history)),
    }


def visualize_debate(results: dict) -> str:
    """
    Create ASCII visualization of the debate network.
    
    Shows agents as nodes with connections showing argument flow.
    """
    lines = [
        "╔══════════════════════════════════════════════════════════╗",
        "║         MULTI-AGENT NEURAL NETWORK DEBATE                ║",
        "╠══════════════════════════════════════════════════════════╣",
        f"║ Problem: {results['problem'][:50]:<50} ║",
        "╠══════════════════════════════════════════════════════════╣",
        "║                                                          ║",
        "║    ┌──────────┐         ┌──────────┐                     ║",
        "║    │ Analyst  │◄───────►│  Critic  │                     ║",
        "║    └────┬─────┘         └────┬─────┘                     ║",
        "║         │                    │                            ║",
        "║         │         ┌──────────┴──────────┐                 ║",
        "║         │         │   Consensus Node    │                 ║",
        "║         │         └──────────┬──────────┘                 ║",
        "║         │                    │                            ║",
        "║    ┌────┴─────┐         ┌────┴─────┐                     ║",
        "║    │Innovator │◄───────►│Validator │                     ║",
        "║    └──────────┘         └──────────┘                     ║",
        "║                                                          ║",
        "╠══════════════════════════════════════════════════════════╣",
        f"║ Rounds: {results.get('rounds_completed', 0):<3}  Agents: 4  Status: {'Complete' if results.get('solution') else 'In Progress':<12} ║",
        "╚══════════════════════════════════════════════════════════╝",
    ]
    return "\n".join(lines)
