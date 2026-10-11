//! Fydraulis Harmonia
// scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 70088809;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When your opponent activates a monster effect on the field (Quick Effect): You can reveal this card in your hand and up to 5 Synchro Monsters in your Extra Deck; apply these effects in sequence based on the total number revealed, also you cannot activate the effects of monsters Special Summoned from the Extra Deck until the end of your next turn, except Synchro Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.effcon)
    effect::set_condition(d, e1, Some(condition(effcon)));
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.effcon(e,tp,eg,ep,ev,re,r,rp)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsMonsterEffect() and Chain.IsTriggeringLocation(ev,LOCATION_MZONE)
    a.rp == 1 - a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && chain::is_triggering(d, a.ev as u8, chain::Prop::Location, &[LOCATION_MZONE as i64]).truthy()
}
/// `s.revealfilter(c)`
fn revealfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSynchroMonster() and not c:IsPublic()
    card::is_synchro_monster(d, c) && !card::is_public(d, c)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local b1=#sg==1
    let b1 = sg.len() == 1;
    // local b2=#sg==3 and sg:IsExists(Card.IsAbleToGrave,1,nil)
    let b2 = sg.len() == 3 && group::is_exists(d, sg, &|d, c| card::is_able_to_grave(d, c, None), 1, NO_EX);
    // local b3=#sg==5 and sg:IsExists(Card.IsAbleToGrave,1,nil) and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    let b3 = sg.len() == 5
        && group::is_exists(d, sg, &|d, c| card::is_able_to_grave(d, c, None), 1, NO_EX)
        && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0;
    // return b1 or b2 or b3
    (b1 || b2 || b3, false)
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetMatchingGroup(s.revealfilter,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, Some(&revealfilter), tp, LOCATION_EXTRA, 0, NO_EX);
    // if chk==0 then return not c:IsPublic() and #g>0 end
    if a.chk == Some(0) {
        return !card::is_public(d, c) && !g.is_empty();
    }
    // local max_reveal_count=math.min(#g,5)
    let max_reveal_count = g.len().min(5);
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,max_reveal_count,s.rescon,1,tp,HINTMSG_CONFIRM,s.rescon)
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(e),
        tp,
        Some(1),
        Some(max_reveal_count),
        Some(&rescon),
        1,
        tp,
        Some(HINTMSG_CONFIRM as u64),
        Some(&rescon),
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.ConfirmCards(1-tp,c)
    duel::confirm_cards(d, 1 - tp, CardSet::from_card(c));
    // Duel.ConfirmCards(1-tp,sg)
    duel::confirm_cards(d, 1 - tp, sg.clone());
    // Duel.ShuffleExtra(tp)
    duel::shuffle_extra(d, tp);
    // for sc in sg:Iter() do
    for sc in group::iter(d, &sg) {
        // sc:CreateEffectRelation(e)
        card::create_effect_relation(d, sc, e);
    }
    // local cd=e:GetChainData()
    // cd.reveal_count=#sg+1
    let reveal_count = sg.len() as i64 + 1;
    chain::set_effect_data(d, e, None, chain::field::CARD, Value::Int(reveal_count));
    // cd.revealed_synchros=sg
    chain::set_effect_data(d, e, None, chain::field::CARD + 1, Value::Group(sg));
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    // local cd=e:GetChainData()
    let reveal_count = chain::effect_data(d, e, None, chain::field::CARD).int();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // if cd.reveal_count>=4 then
    if reveal_count >= 4 {
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_EXTRA)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_EXTRA as i32);
    }
    // if cd.reveal_count==6 then
    if reveal_count == 6 {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Duel.GetFieldGroup(tp,0,LOCATION_MZONE),1,tp,0)
        let og = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(og), 1, tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local cd=e:GetChainData()
    let reveal_count = chain::effect_data(d, e, None, chain::field::CARD).int();
    // local break_chk=false
    let mut break_chk = false;
    // if cd.reveal_count>=2 and c:IsRelateToEffect(e) then
    if reveal_count >= 2 && card::is_relate_to_effect(d, c, e) {
        // --● 2+: Special Summon this card
        // break_chk=true
        break_chk = true;
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // if cd.reveal_count>=4 then
    if reveal_count >= 4 {
        // --● 4+: Send 1 of the revealed Synchro Monsters to the GY
        // cd.revealed_synchros:Match(Card.IsRelateToEffect,nil,e)
        let mut rs = chain::effect_data(d, e, None, chain::field::CARD + 1).as_group().cloned().nil();
        group::match_(d, &mut rs, &|d, x| card::is_relate_to_effect(d, x, e), NO_EX);
        // the stored group changed: store it again
        chain::set_effect_data(d, e, None, chain::field::CARD + 1, Value::Group(rs.clone()));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=cd.revealed_synchros:FilterSelect(tp,Card.IsAbleToGrave,1,1,nil)
        let g = group::filter_select(d, &rs, tp, &|d, x| card::is_able_to_grave(d, x, None), 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // if break_chk then Duel.BreakEffect() end
            if break_chk {
                duel::break_effect(d);
            }
            // break_chk=true
            break_chk = true;
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    }
    // if cd.reveal_count==6 then
    if reveal_count == 6 {
        // --● 6: Destroy 1 monster your opponent controls
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // if break_chk then Duel.BreakEffect() end
            if break_chk {
                duel::break_effect(d);
            }
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
    // local reset_count=Duel.IsTurnPlayer(tp) and 2 or 1
    let reset_count: u16 = if duel::is_turn_player(d, tp) { 2 } else { 1 };
    // --You cannot activate the effects of monsters Special Summoned from the Extra Deck until the end of your next turn, except Synchro Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,reset_count)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, reset_count);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    // return re:IsMonsterEffect() and rc:IsOnField() and rc:IsSummonLocation(LOCATION_EXTRA) and not rc:IsSynchroMonster()
    effect::is_monster_effect(d, re) && card::is_on_field(d, rc) && card::is_summon_location(d, rc, LOCATION_EXTRA) && !card::is_synchro_monster(d, rc)
}
