//! SPYGAL Misty
use super::prelude::*;

pub const ID: u32 = 94096018;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SPYRAL_SUPER_AGENT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reveal the top card of your opponent's Deck, and if you do, draw 1 card if it is a card of the declared type
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1a, CATEGORY_DRAW);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY + EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetTarget(s.drtg)
    effect::set_target(d, e1a, Some(cost_target(drtg)));
    // e1a:SetOperation(s.drop)
    effect::set_operation(d, e1a, Some(operation(drop_op)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Return 1 "SPYRAL Super Agent" you control and 1 monster your opponent controls to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.rthtg)
    effect::set_target(d, e2, Some(cost_target(rthtg)));
    // e2:SetOperation(s.rthop)
    effect::set_operation(d, e2, Some(operation(rthop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>0 end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1) && duel::get_field_group_count(d, tp, 0, LOCATION_DECK) > 0;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CARDTYPE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CARDTYPE as u64);
    // e:SetLabel(Duel.SelectOption(tp,DECLTYPE_MONSTER,DECLTYPE_SPELL,DECLTYPE_TRAP))
    let opt = duel::select_option(d, tp, true, &[DECLTYPE_MONSTER as u64, DECLTYPE_SPELL as u64, DECLTYPE_TRAP as u64]);
    effect::set_label(d, e, &[opt as i64]);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)==0 then return end
    if duel::get_field_group_count(d, tp, 0, LOCATION_DECK) == 0 {
        return;
    }
    // local opt=e:GetLabel()
    let opt = effect::get_label(d, e);
    // Duel.ConfirmDecktop(1-tp,1)
    let _ = duel::confirm_decktop(d, 1 - tp, 1);
    // local top_c=Duel.GetDecktopGroup(1-tp,1):GetFirst()
    let deck_top = duel::get_decktop_group(d, 1 - tp, 1);
    let top_c = group::get_first(d, &deck_top);
    // if (opt==0 and top_c:IsMonster()) or (opt==1 and top_c:IsSpell()) or (opt==2 and top_c:IsTrap()) then
    let matched = (opt == 0 && card::is_monster(d, top_c.nil()))
        || (opt == 1 && card::is_spell(d, top_c.nil()))
        || (opt == 2 && card::is_trap(d, top_c.nil()));
    if matched {
        // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
        let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
        // Duel.Draw(p,d,REASON_EFFECT)
        duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
    }
}
/// `s.rthfilter(c,e,tp)`
fn rthfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return ((c:IsCode(CARD_SPYRAL_SUPER_AGENT) and c:IsFaceup()) or c:IsControler(1-tp)) and c:IsAbleToHand()
    //     and c:IsCanBeEffectTarget(e)
    ((card::is_code(d, c, &[CARD_SPYRAL_SUPER_AGENT]) && card::is_faceup(d, c)) || card::is_controler(d, c, 1 - tp))
        && card::is_able_to_hand(d, c, None)
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.rthfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| rthfilter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    let dp = aux::dpcheck(card_controler_value);
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),0) end
    if a.chk == Some(0) {
        return g.len() >= 2
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&dp), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),1,tp,HINTMSG_RTOHAND)
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&dp), 1, tp, Some(HINTMSG_RTOHAND as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), 2, tp, 0);
    true
}
/// `Card.GetControler` as the value of `aux.dpcheck`
fn card_controler_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_controler(d, c) as i64
}
/// `s.rthop(e,tp,eg,ep,ev,re,r,rp)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==2 and tg:FilterCount(Card.IsMonster,nil)==2 then
    if tg.len() == 2 && group::filter_count(d, &tg, &card::is_monster, NO_EX) == 2 {
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
