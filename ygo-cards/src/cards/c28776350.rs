//! Akashic Magician
use super::prelude::*;

pub const ID: u32 = 28776350;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon Procedure
    // Link.AddProcedure(c,s.filter,2,nil,s.spcheck)
    link::add_procedure(d, c, Some(mat_filter(filter)), 2, None, Some(group_check(spcheck)), None, None);
    // Can only be Link Summoned once per turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.regcon)
    effect::set_condition(d, e1, Some(condition(regcon)));
    // e1:SetOperation(s.regop)
    effect::set_operation(d, e1, Some(operation(regop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return to the hand all monsters this card points to
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Excavate cards from the top f your Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DECKDES)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DECKDES);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.actg)
    effect::set_target(d, e3, Some(cost_target(actg)));
    // e3:SetOperation(s.acop)
    effect::set_operation(d, e3, Some(operation(acop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c,lc,sumtype,tp)`
fn filter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return not c:IsType(TYPE_TOKEN,lc,sumtype,tp)
    !card::is_type_ctx(d, c, TYPE_TOKEN, lc, sumtype, Some(tp))
}
/// `s.spcheck(g,lc,tp)`
fn spcheck(d: &mut Duel, g: &CardSet, lc: CardId, _sumtype: u64, tp: u8) -> bool {
    // return g:CheckSameProperty(Card.GetRace,lc,SUMMON_TYPE_LINK,tp)
    group::check_same_property(d, g, &|d, c| card::get_race_ctx(d, c, Some(lc), SUMMON_TYPE_LINK as u64, Some(tp))).0
}
/// `s.regcon(e,tp,...)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
/// `s.regop(e,tp,...)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return c:IsCode(id) and sumtype&SUMMON_TYPE_LINK==SUMMON_TYPE_LINK
    let c = a.c(1);
    let sumtype = a.int(3) as u64;
    card::is_code(d, c, &[ID]) && (sumtype & SUMMON_TYPE_LINK as u64) == SUMMON_TYPE_LINK as u64
}
/// `s.thcon(e,tp,...)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local lg=e:GetHandler():GetLinkedGroup():Filter(Card.IsAbleToHand,nil)
    let c = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, c);
    let lg = group::filter(d, &lg, &|d, x| card::is_able_to_hand(d, x, None), NO_EX);
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,lg,#lg,0,0)
    let n = lg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(lg), n, 0, 0);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local lg=e:GetHandler():GetLinkedGroup():Filter(Card.IsAbleToHand,nil)
    let c = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, c);
    let lg = group::filter(d, &lg, &|d, x| card::is_able_to_hand(d, x, None), NO_EX);
    // Duel.SendtoHand(lg,nil,REASON_EFFECT)
    duel::send_to_hand(d, lg, None, REASON_EFFECT, None);
}
/// `s.actg(e,tp,...,chk)`
fn actg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local lg=c:GetMutualLinkedGroup():Filter(Card.IsMonster,nil)
        let lg = card::get_mutual_linked_group(d, c);
        let lg = group::filter(d, &lg, &|d, x| card::is_monster(d, x), NO_EX);
        // local ct=lg:GetSum(Card.GetLink)
        let ct = group::get_sum(d, &lg, &|d, x| card::get_link(d, x) as i64);
        // if ct<=0 or not Duel.IsPlayerCanDiscardDeck(tp,ct) then return false end
        if ct <= 0 || !duel::is_player_can_discard_deck(d, a.tp, ct as u32) {
            return false;
        }
        // local g=Duel.GetDecktopGroup(tp,ct)
        let g = duel::get_decktop_group(d, a.tp, ct as u32);
        // return g:FilterCount(Card.IsAbleToHand,nil)>0
        return group::filter_count(d, &g, &|d, x| card::is_able_to_hand(d, x, None), NO_EX) > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CODE as u64);
    // s.announce_filter={TYPE_EXTRA,OPCODE_ISTYPE,OPCODE_NOT}
    // (only set and read by this function, so it is a local)
    let announce_filter = [TYPE_EXTRA as u64, OPCODE_ISTYPE, OPCODE_NOT];
    // local ac=Duel.AnnounceCard(tp,table.unpack(s.announce_filter))
    let ac = duel::announce_card(d, a.tp, &announce_filter);
    // Duel.SetTargetParam(ac)
    duel::set_target_param(d, ac as u32);
    // Duel.SetOperationInfo(0,CATEGORY_ANNOUNCE,nil,0,tp,ANNOUNCE_CARD_FILTER)
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, a.tp, ANNOUNCE_CARD_FILTER as i32);
    true
}
/// `s.thfilter(c,code)`
fn thfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsCode(code) and c:IsAbleToHand()
    card::is_code(d, c, &[code]) && card::is_able_to_hand(d, c, None)
}
/// `s.acop(e,tp,...)`
fn acop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lg=c:GetMutualLinkedGroup()
    let lg = card::get_mutual_linked_group(d, c);
    // local ct=lg:GetSum(Card.GetLink)
    let ct = group::get_sum(d, &lg, &|d, x| card::get_link(d, x) as i64);
    // if ct<=0 or not Duel.IsPlayerCanDiscardDeck(tp,ct) then return end
    if ct <= 0 || !duel::is_player_can_discard_deck(d, a.tp, ct as u32) {
        return;
    }
    // Duel.ConfirmDecktop(tp,ct)
    duel::confirm_decktop(d, a.tp, ct as u32);
    // local g=Duel.GetDecktopGroup(tp,ct)
    let mut g = duel::get_decktop_group(d, a.tp, ct as u32);
    // local ac=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let ac = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // local hg=g:Filter(s.thfilter,nil,ac)
    let hg = group::filter(d, &g, &|d, x| thfilter(d, x, ac), NO_EX);
    // g:Sub(hg)
    group::remove_card(&mut g, &hg);
    // if #hg~=0 then
    if !hg.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoHand(hg,nil,REASON_EFFECT)
        duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,hg)
        duel::confirm_cards(d, 1 - a.tp, hg);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
    }
    // if #g~=0 then
    if !g.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_EXCAVATE)
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_EXCAVATE, None, None);
    }
}
