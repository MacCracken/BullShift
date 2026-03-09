import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import '../core_trading/trading_provider.dart';

class NotesPanel extends StatefulWidget {
  final String symbol;

  const NotesPanel({
    super.key,
    required this.symbol,
  });

  @override
  State<NotesPanel> createState() => _NotesPanelState();
}

class _NotesPanelState extends State<NotesPanel> {
  final TextEditingController _noteController = TextEditingController();
  final ScrollController _scrollController = ScrollController();

  @override
  void dispose() {
    _noteController.dispose();
    _scrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Consumer<TradingProvider>(
      builder: (context, tradingProvider, child) {
        final notes = tradingProvider.getNotesForSymbol(widget.symbol);

        return Container(
          margin: const EdgeInsets.all(8),
          padding: const EdgeInsets.all(16),
          decoration: BoxDecoration(
            color: const Color(0xFF263238),
            borderRadius: BorderRadius.circular(8),
          ),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                children: [
                  const Icon(Icons.note_alt, color: Colors.white, size: 20),
                  const SizedBox(width: 8),
                  Text(
                    'Notes - ${widget.symbol}',
                    style: const TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                    ),
                  ),
                  const Spacer(),
                  IconButton(
                    icon: const Icon(Icons.clear_all,
                        color: Colors.white, size: 16),
                    onPressed: () => _clearAllNotes(tradingProvider),
                    tooltip: 'Clear all notes',
                  ),
                ],
              ),
              const SizedBox(height: 12),
              // Note input
              Row(
                children: [
                  Expanded(
                    child: TextField(
                      controller: _noteController,
                      decoration: const InputDecoration(
                        hintText: 'Add trading note...',
                        hintStyle: TextStyle(color: Colors.grey),
                        border: OutlineInputBorder(),
                        contentPadding:
                            EdgeInsets.symmetric(horizontal: 12, vertical: 8),
                      ),
                      style: const TextStyle(color: Colors.white),
                      maxLines: 2,
                      minLines: 1,
                    ),
                  ),
                  const SizedBox(width: 8),
                  ElevatedButton(
                    onPressed: () => _addNote(tradingProvider),
                    style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.blue,
                      padding: const EdgeInsets.symmetric(
                          horizontal: 16, vertical: 12),
                    ),
                    child: const Text('Add'),
                  ),
                ],
              ),
              const SizedBox(height: 12),
              // Quick note templates
              SizedBox(
                height: 30,
                child: ListView(
                  scrollDirection: Axis.horizontal,
                  children: [
                    _buildQuickNote(
                        'Earnings beat expectations', tradingProvider),
                    _buildQuickNote('Technical breakout', tradingProvider),
                    _buildQuickNote('Volume spike detected', tradingProvider),
                    _buildQuickNote('News catalyst', tradingProvider),
                    _buildQuickNote('Risk management', tradingProvider),
                  ],
                ),
              ),
              const SizedBox(height: 12),
              // Notes list
              Expanded(
                child: notes.isEmpty
                    ? const Center(
                        child: Text(
                          'No notes yet. Add your first trading note!',
                          style: TextStyle(color: Colors.grey),
                        ),
                      )
                    : ListView.builder(
                        controller: _scrollController,
                        itemCount: notes.length,
                        itemBuilder: (context, index) {
                          final note = notes[index];
                          return NoteCard(
                            note: note,
                            onDelete: () =>
                                _deleteNote(tradingProvider, note['id']),
                            onEdit: () => _editNote(tradingProvider, note),
                          );
                        },
                      ),
              ),
            ],
          ),
        );
      },
    );
  }

  Widget _buildQuickNote(String template, TradingProvider tradingProvider) {
    return Padding(
      padding: const EdgeInsets.only(right: 8),
      child: ActionChip(
        label: Text(
          template,
          style: const TextStyle(fontSize: 12),
        ),
        backgroundColor: const Color(0xFF37474F),
        labelStyle: const TextStyle(color: Colors.white),
        onPressed: () {
          _noteController.text = template;
          _addNote(tradingProvider);
        },
      ),
    );
  }

  void _addNote(TradingProvider tradingProvider) {
    final noteText = _noteController.text.trim();
    if (noteText.isEmpty) return;

    tradingProvider.addNote(
      symbol: widget.symbol,
      note: noteText,
      tags: _extractTags(noteText),
    );

    _noteController.clear();
    _scrollToBottom();
  }

  void _deleteNote(TradingProvider tradingProvider, String noteId) {
    tradingProvider.deleteNote(noteId);
  }

  void _editNote(TradingProvider tradingProvider, Map<String, dynamic> note) {
    _noteController.text = note['note'];
    tradingProvider.deleteNote(note['id']);
  }

  void _clearAllNotes(TradingProvider tradingProvider) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Clear All Notes'),
        content: const Text(
            'Are you sure you want to delete all notes for this symbol?'),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              tradingProvider.clearNotesForSymbol(widget.symbol);
              Navigator.of(context).pop();
            },
            style: TextButton.styleFrom(foregroundColor: Colors.red),
            child: const Text('Clear'),
          ),
        ],
      ),
    );
  }

  List<String> _extractTags(String note) {
    final tags = <String>[];
    final tagPatterns = [
      '#earnings',
      '#technical',
      '#fundamental',
      '#news',
      '#risk',
      '#breakout',
      '#support',
      '#resistance'
    ];

    for (final pattern in tagPatterns) {
      if (note.toLowerCase().contains(pattern)) {
        tags.add(pattern);
      }
    }

    return tags;
  }

  void _scrollToBottom() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (_scrollController.hasClients) {
        _scrollController.animateTo(
          _scrollController.position.maxScrollExtent,
          duration: const Duration(milliseconds: 300),
          curve: Curves.easeOut,
        );
      }
    });
  }
}

class NoteCard extends StatelessWidget {
  final Map<String, dynamic> note;
  final VoidCallback onDelete;
  final VoidCallback onEdit;

  const NoteCard({
    super.key,
    required this.note,
    required this.onDelete,
    required this.onEdit,
  });

  @override
  Widget build(BuildContext context) {
    final noteText = note['note'] as String;
    final timestamp = note['timestamp'] as DateTime;
    final tags = (note['tags'] as List<String>?) ?? [];

    return Card(
      color: const Color(0xFF37474F),
      margin: const EdgeInsets.only(bottom: 8),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(
                  child: Text(
                    noteText,
                    style: const TextStyle(
                      color: Colors.white,
                      fontSize: 14,
                    ),
                  ),
                ),
                PopupMenuButton<String>(
                  icon: const Icon(Icons.more_vert,
                      color: Colors.white, size: 16),
                  itemBuilder: (context) => [
                    const PopupMenuItem(
                      value: 'edit',
                      child: Row(
                        children: [
                          Icon(Icons.edit, size: 16),
                          SizedBox(width: 8),
                          Text('Edit'),
                        ],
                      ),
                    ),
                    const PopupMenuItem(
                      value: 'delete',
                      child: Row(
                        children: [
                          Icon(Icons.delete, size: 16, color: Colors.red),
                          SizedBox(width: 8),
                          Text('Delete', style: TextStyle(color: Colors.red)),
                        ],
                      ),
                    ),
                  ],
                  onSelected: (value) {
                    switch (value) {
                      case 'edit':
                        onEdit();
                        break;
                      case 'delete':
                        onDelete();
                        break;
                    }
                  },
                ),
              ],
            ),
            if (tags.isNotEmpty) ...[
              const SizedBox(height: 8),
              Wrap(
                spacing: 4,
                runSpacing: 4,
                children: tags.map((tag) => _buildTag(tag)).toList(),
              ),
            ],
            const SizedBox(height: 8),
            Row(
              children: [
                Text(
                  _formatTimestamp(timestamp),
                  style: const TextStyle(
                    color: Colors.grey,
                    fontSize: 12,
                  ),
                ),
                const Spacer(),
                if (note['isPinned'] == true)
                  const Icon(
                    Icons.push_pin,
                    color: Colors.yellow,
                    size: 12,
                  ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildTag(String tag) {
    Color tagColor;
    switch (tag.toLowerCase()) {
      case '#earnings':
        tagColor = Colors.green;
        break;
      case '#technical':
        tagColor = Colors.blue;
        break;
      case '#fundamental':
        tagColor = Colors.purple;
        break;
      case '#news':
        tagColor = Colors.orange;
        break;
      case '#risk':
        tagColor = Colors.red;
        break;
      case '#breakout':
        tagColor = Colors.yellow;
        break;
      default:
        tagColor = Colors.grey;
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
      decoration: BoxDecoration(
        color: tagColor.withOpacity(0.3),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: tagColor.withOpacity(0.5)),
      ),
      child: Text(
        tag,
        style: TextStyle(
          color: tagColor,
          fontSize: 10,
          fontWeight: FontWeight.bold,
        ),
      ),
    );
  }

  String _formatTimestamp(DateTime timestamp) {
    final now = DateTime.now();
    final difference = now.difference(timestamp);

    if (difference.inMinutes < 1) {
      return 'Just now';
    } else if (difference.inHours < 1) {
      return '${difference.inMinutes}m ago';
    } else if (difference.inDays < 1) {
      return '${difference.inHours}h ago';
    } else if (difference.inDays < 7) {
      return '${difference.inDays}d ago';
    } else {
      return '${timestamp.day}/${timestamp.month}/${timestamp.year}';
    }
  }
}
