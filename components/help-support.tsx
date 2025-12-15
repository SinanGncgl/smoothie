"use client";

import { motion, AnimatePresence } from "framer-motion";
import {
  HelpCircle,
  MessageSquarePlus,
  Lightbulb,
  Bug,
  Send,
  Loader2,
  CheckCircle2,
  Clock,
  AlertCircle,
  BookOpen,
  ExternalLink,
  Mail,
  History,
} from "lucide-react";
import { useState, useEffect, useCallback } from "react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Textarea } from "@/components/ui/textarea";
import { useToast } from "@/hooks/use-toast";
import {
  feedbackApi,
  isTauri,
  type Feedback,
  type CreateFeedbackRequest,
} from "@/lib/tauri";

const FEEDBACK_TYPES = [
  {
    id: "feature_request",
    label: "Feature Request",
    icon: Lightbulb,
    color: "text-yellow-500",
  },
  { id: "bug_report", label: "Bug Report", icon: Bug, color: "text-red-500" },
  {
    id: "general_feedback",
    label: "General Feedback",
    icon: MessageSquarePlus,
    color: "text-blue-500",
  },
  {
    id: "question",
    label: "Question",
    icon: HelpCircle,
    color: "text-purple-500",
  },
];

const CATEGORIES = [
  "Monitor Management",
  "Profile Switching",
  "App Launching",
  "Window Positioning",
  "Automation",
  "Performance",
  "UI/UX",
  "Other",
];

const PRIORITIES = [
  { id: "low", label: "Low", color: "bg-gray-500" },
  { id: "medium", label: "Medium", color: "bg-yellow-500" },
  { id: "high", label: "High", color: "bg-orange-500" },
  { id: "critical", label: "Critical", color: "bg-red-500" },
];

const FAQ_ITEMS = [
  {
    question: "How do I create a new profile?",
    answer:
      "Go to the Dashboard and click the 'New Profile' button. Enter a name, description, and optionally capture your current monitor layout.",
  },
  {
    question: "How do I save my current monitor arrangement?",
    answer:
      "Go to System Capture, click 'Capture Layout' to snapshot your current setup, then save it to a profile.",
  },
  {
    question: "Can I automatically switch profiles?",
    answer:
      "Yes! Use the Automation Rules in Settings to set up triggers based on time of day or connected monitors.",
  },
  {
    question: "How do I restore a saved layout?",
    answer:
      "Click the play button on any profile card in the Dashboard to activate it and restore the saved configuration.",
  },
  {
    question: "Why aren't my monitor positions saving?",
    answer:
      "Make sure you grant Smoothie the necessary permissions in System Preferences > Security & Privacy > Accessibility.",
  },
];

export function HelpSupport() {
  const [activeTab, setActiveTab] = useState("submit");
  const [feedbackType, setFeedbackType] = useState("feature_request");
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [category, setCategory] = useState("");
  const [priority, setPriority] = useState("medium");
  const [contactEmail, setContactEmail] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [submitSuccess, setSubmitSuccess] = useState(false);
  const [previousFeedback, setPreviousFeedback] = useState<Feedback[]>([]);
  const [isLoadingHistory, setIsLoadingHistory] = useState(false);
  const { toast } = useToast();

  const loadFeedbackHistory = useCallback(async () => {
    if (!isTauri()) {
      // Mock data for development
      setPreviousFeedback([
        {
          id: "1",
          userId: "1",
          feedbackType: "feature_request",
          title: "Add dark mode toggle in menu bar",
          description: "Would be nice to quickly toggle dark mode",
          priority: "medium",
          status: "open",
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
        },
      ]);
      return;
    }

    setIsLoadingHistory(true);
    try {
      const feedback = await feedbackApi.getFeedback(undefined, undefined, 20);
      setPreviousFeedback(feedback);
    } catch {
      // Silent failure for feedback history loading
    } finally {
      setIsLoadingHistory(false);
    }
  }, []);

  useEffect(() => {
    if (activeTab === "history") {
      loadFeedbackHistory();
    }
  }, [activeTab, loadFeedbackHistory]);

  const handleSubmit = async () => {
    if (!title.trim() || !description.trim()) {
      toast({
        title: "Missing Information",
        description: "Please provide a title and description",
        variant: "destructive",
      });
      return;
    }

    setIsSubmitting(true);
    try {
      const req: CreateFeedbackRequest = {
        feedbackType,
        title: title.trim(),
        description: description.trim(),
        priority,
        category: category || undefined,
        contactEmail: contactEmail.trim() || undefined,
      };

      if (isTauri()) {
        await feedbackApi.submitFeedback(req);
      }

      setSubmitSuccess(true);
      toast({
        title: "Feedback Submitted",
        description: "Thank you for your feedback! We'll review it soon.",
      });

      // Reset form after delay
      setTimeout(() => {
        setTitle("");
        setDescription("");
        setCategory("");
        setPriority("medium");
        setContactEmail("");
        setSubmitSuccess(false);
      }, 2000);
    } catch {
      toast({
        title: "Submission Failed",
        description: "Failed to submit feedback. Please try again.",
        variant: "destructive",
      });
    } finally {
      setIsSubmitting(false);
    }
  };

  const getStatusBadge = (status: string) => {
    switch (status) {
      case "open":
        return (
          <Badge variant="outline" className="gap-1">
            <Clock className="w-3 h-3" />
            Open
          </Badge>
        );
      case "in_progress":
        return (
          <Badge className="gap-1 bg-blue-500">
            <Loader2 className="w-3 h-3 animate-spin" />
            In Progress
          </Badge>
        );
      case "resolved":
        return (
          <Badge className="gap-1 bg-green-500">
            <CheckCircle2 className="w-3 h-3" />
            Resolved
          </Badge>
        );
      case "closed":
        return (
          <Badge variant="secondary" className="gap-1">
            Closed
          </Badge>
        );
      default:
        return <Badge variant="outline">{status}</Badge>;
    }
  };

  const getFeedbackTypeInfo = (type: string) => {
    return FEEDBACK_TYPES.find((t) => t.id === type) || FEEDBACK_TYPES[0];
  };

  const containerVariants = {
    hidden: { opacity: 0 },
    visible: {
      opacity: 1,
      transition: { staggerChildren: 0.08, delayChildren: 0.1 },
    },
  };

  const itemVariants = {
    hidden: { opacity: 0, y: 10 },
    visible: { opacity: 1, y: 0, transition: { duration: 0.3 } },
  };

  return (
    <div className="h-full flex flex-col p-8 gap-6 overflow-y-auto">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: -10 }}
        animate={{ opacity: 1, y: 0 }}
      >
        <h3 className="text-lg font-semibold text-foreground mb-1 flex items-center gap-2">
          <MessageSquarePlus className="w-5 h-5" />
          Feedback & Feature Requests
        </h3>
        <p className="text-sm text-muted-foreground">
          Share your ideas, report issues, or ask questions to help us improve
          Smoothie
        </p>
      </motion.div>

      <Tabs value={activeTab} onValueChange={setActiveTab} className="flex-1">
        <TabsList className="grid w-full grid-cols-3 mb-6">
          <TabsTrigger value="submit" className="gap-2">
            <MessageSquarePlus className="w-4 h-4" />
            Submit Feedback
          </TabsTrigger>
          <TabsTrigger value="faq" className="gap-2">
            <BookOpen className="w-4 h-4" />
            FAQ
          </TabsTrigger>
          <TabsTrigger value="history" className="gap-2">
            <History className="w-4 h-4" />
            My Submissions
          </TabsTrigger>
        </TabsList>

        {/* Submit Feedback Tab */}
        <TabsContent value="submit">
          <motion.div
            variants={containerVariants}
            initial="hidden"
            animate="visible"
            className="space-y-6"
          >
            {/* Feedback Type Selection */}
            <motion.div variants={itemVariants}>
              <Card>
                <CardHeader>
                  <CardTitle className="text-base">
                    What would you like to share?
                  </CardTitle>
                  <CardDescription>Select the type of feedback</CardDescription>
                </CardHeader>
                <CardContent>
                  <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
                    {FEEDBACK_TYPES.map((type) => {
                      const Icon = type.icon;
                      const isSelected = feedbackType === type.id;
                      return (
                        <motion.button
                          key={type.id}
                          onClick={() => setFeedbackType(type.id)}
                          className={`p-4 rounded-lg border-2 transition-all ${
                            isSelected
                              ? "border-primary bg-primary/10"
                              : "border-border hover:border-primary/50 hover:bg-muted/50"
                          }`}
                          whileHover={{ scale: 1.02 }}
                          whileTap={{ scale: 0.98 }}
                        >
                          <Icon
                            className={`w-6 h-6 mx-auto mb-2 ${type.color}`}
                          />
                          <span className="text-sm font-medium">
                            {type.label}
                          </span>
                        </motion.button>
                      );
                    })}
                  </div>
                </CardContent>
              </Card>
            </motion.div>

            {/* Feedback Form */}
            <motion.div variants={itemVariants}>
              <Card>
                <CardHeader>
                  <CardTitle className="text-base">Details</CardTitle>
                  <CardDescription>
                    Provide as much detail as possible
                  </CardDescription>
                </CardHeader>
                <CardContent className="space-y-4">
                  <div className="space-y-2">
                    <Label htmlFor="title">Title *</Label>
                    <Input
                      id="title"
                      placeholder="Brief summary of your feedback"
                      value={title}
                      onChange={(e) => setTitle(e.target.value)}
                      disabled={isSubmitting}
                    />
                  </div>

                  <div className="space-y-2">
                    <Label htmlFor="description">Description *</Label>
                    <Textarea
                      id="description"
                      placeholder="Describe your feedback in detail. For bug reports, include steps to reproduce."
                      value={description}
                      onChange={(e) => setDescription(e.target.value)}
                      rows={5}
                      disabled={isSubmitting}
                    />
                  </div>

                  <div className="grid grid-cols-2 gap-4">
                    <div className="space-y-2">
                      <Label>Category</Label>
                      <Select
                        value={category}
                        onValueChange={setCategory}
                        disabled={isSubmitting}
                      >
                        <SelectTrigger>
                          <SelectValue placeholder="Select category" />
                        </SelectTrigger>
                        <SelectContent>
                          {CATEGORIES.map((cat) => (
                            <SelectItem key={cat} value={cat}>
                              {cat}
                            </SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                    </div>

                    <div className="space-y-2">
                      <Label>Priority</Label>
                      <Select
                        value={priority}
                        onValueChange={setPriority}
                        disabled={isSubmitting}
                      >
                        <SelectTrigger>
                          <SelectValue placeholder="Select priority" />
                        </SelectTrigger>
                        <SelectContent>
                          {PRIORITIES.map((p) => (
                            <SelectItem key={p.id} value={p.id}>
                              <div className="flex items-center gap-2">
                                <div
                                  className={`w-2 h-2 rounded-full ${p.color}`}
                                />
                                {p.label}
                              </div>
                            </SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                    </div>
                  </div>

                  <div className="space-y-2">
                    <Label htmlFor="email" className="flex items-center gap-2">
                      <Mail className="w-4 h-4" />
                      Contact Email (optional)
                    </Label>
                    <Input
                      id="email"
                      type="email"
                      placeholder="your@email.com - We'll notify you of updates"
                      value={contactEmail}
                      onChange={(e) => setContactEmail(e.target.value)}
                      disabled={isSubmitting}
                    />
                  </div>
                </CardContent>
              </Card>
            </motion.div>

            {/* Submit Button */}
            <motion.div variants={itemVariants}>
              <AnimatePresence mode="wait">
                {submitSuccess ? (
                  <motion.div
                    initial={{ opacity: 0, scale: 0.9 }}
                    animate={{ opacity: 1, scale: 1 }}
                    exit={{ opacity: 0, scale: 0.9 }}
                    className="flex items-center justify-center gap-2 py-4 text-green-500"
                  >
                    <CheckCircle2 className="w-6 h-6" />
                    <span className="font-medium">
                      Feedback submitted successfully!
                    </span>
                  </motion.div>
                ) : (
                  <Button
                    onClick={handleSubmit}
                    disabled={
                      isSubmitting || !title.trim() || !description.trim()
                    }
                    className="w-full gap-2"
                    size="lg"
                  >
                    {isSubmitting ? (
                      <>
                        <Loader2 className="w-4 h-4 animate-spin" />
                        Submitting...
                      </>
                    ) : (
                      <>
                        <Send className="w-4 h-4" />
                        Submit Feedback
                      </>
                    )}
                  </Button>
                )}
              </AnimatePresence>
            </motion.div>
          </motion.div>
        </TabsContent>

        {/* FAQ Tab */}
        <TabsContent value="faq">
          <motion.div
            variants={containerVariants}
            initial="hidden"
            animate="visible"
            className="space-y-4"
          >
            {FAQ_ITEMS.map((item, index) => (
              <motion.div key={index} variants={itemVariants}>
                <Card>
                  <CardHeader className="pb-2">
                    <CardTitle className="text-base flex items-center gap-2">
                      <HelpCircle className="w-4 h-4 text-primary" />
                      {item.question}
                    </CardTitle>
                  </CardHeader>
                  <CardContent>
                    <p className="text-sm text-muted-foreground">
                      {item.answer}
                    </p>
                  </CardContent>
                </Card>
              </motion.div>
            ))}

            <motion.div variants={itemVariants}>
              <Card className="bg-muted/50">
                <CardContent className="pt-6">
                  <div className="flex items-center justify-between">
                    <div>
                      <p className="font-medium">Still need help?</p>
                      <p className="text-sm text-muted-foreground">
                        Submit a feedback form or check our documentation
                      </p>
                    </div>
                    <Button
                      variant="outline"
                      className="gap-2"
                      onClick={() => setActiveTab("submit")}
                    >
                      <MessageSquarePlus className="w-4 h-4" />
                      Ask a Question
                    </Button>
                  </div>
                </CardContent>
              </Card>
            </motion.div>
          </motion.div>
        </TabsContent>

        {/* History Tab */}
        <TabsContent value="history">
          <motion.div
            variants={containerVariants}
            initial="hidden"
            animate="visible"
            className="space-y-4"
          >
            {isLoadingHistory ? (
              <div className="flex items-center justify-center py-12">
                <Loader2 className="w-6 h-6 animate-spin text-muted-foreground" />
              </div>
            ) : previousFeedback.length === 0 ? (
              <motion.div variants={itemVariants}>
                <Card className="bg-muted/30">
                  <CardContent className="pt-6 text-center">
                    <History className="w-12 h-12 mx-auto mb-4 text-muted-foreground opacity-50" />
                    <p className="text-muted-foreground">No submissions yet</p>
                    <p className="text-sm text-muted-foreground mt-1">
                      Your feedback history will appear here
                    </p>
                    <Button
                      variant="outline"
                      className="mt-4 gap-2"
                      onClick={() => setActiveTab("submit")}
                    >
                      <MessageSquarePlus className="w-4 h-4" />
                      Submit Your First Feedback
                    </Button>
                  </CardContent>
                </Card>
              </motion.div>
            ) : (
              previousFeedback.map((feedback) => {
                const typeInfo = getFeedbackTypeInfo(feedback.feedbackType);
                const TypeIcon = typeInfo.icon;
                return (
                  <motion.div key={feedback.id} variants={itemVariants}>
                    <Card>
                      <CardHeader className="pb-2">
                        <div className="flex items-start justify-between">
                          <div className="flex items-center gap-2">
                            <TypeIcon className={`w-4 h-4 ${typeInfo.color}`} />
                            <CardTitle className="text-base">
                              {feedback.title}
                            </CardTitle>
                          </div>
                          {getStatusBadge(feedback.status)}
                        </div>
                        <CardDescription className="flex items-center gap-2 mt-1">
                          <Badge variant="outline" className="text-xs">
                            {typeInfo.label}
                          </Badge>
                          {feedback.category && (
                            <Badge variant="secondary" className="text-xs">
                              {feedback.category}
                            </Badge>
                          )}
                          <span className="text-xs">
                            {new Date(feedback.createdAt).toLocaleDateString()}
                          </span>
                        </CardDescription>
                      </CardHeader>
                      <CardContent>
                        <p className="text-sm text-muted-foreground line-clamp-2">
                          {feedback.description}
                        </p>
                      </CardContent>
                    </Card>
                  </motion.div>
                );
              })
            )}
          </motion.div>
        </TabsContent>
      </Tabs>
    </div>
  );
}
