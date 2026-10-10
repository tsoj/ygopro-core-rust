//! D/D Savant Kepler
use super::prelude::*;

pub const ID: u32 = 11609969;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DD, SET_DARK_CONTRACT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Cannot Pendulum Summon monsters, except "D/D" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,_c,tp,sumtp,sumpos) return not _c:IsSetCard(SET_DD) and (sumtp&SUMMON_TYPE_PENDULUM)==SUMMON_TYPE_PENDULUM end)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Reduce this card's Pendulum Scale by 2
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(_,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(sccon)));
    // e2:SetTarget(s.sctg)
    effect::set_target(d, e2, Some(cost_target(sctg)));
    // e2:SetOperation(s.scop)
    effect::set_operation(d, e2, Some(operation(scop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Activate 1 of these effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function(e,_c,tp,sumtp,sumpos) return not _c:IsSetCard(SET_DD) and (sumtp&SUMMON_TYPE_PENDULUM)==SUMMON_TYPE_PENDULUM end
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let sumtp = a.int(3) as u64;
    !card::is_set_card(d, c, &[SET_DD as u16]) && (sumtp & SUMMON_TYPE_PENDULUM as u64) == SUMMON_TYPE_PENDULUM as u64
}
// function(_,tp) return Duel.IsTurnPlayer(tp) end
fn sccon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
// s.desfilter(c,lvl)
fn desfilter(d: &mut Duel, c: CardId, lvl: u32) -> bool {
    // return c:IsFaceup() and not c:IsSetCard(SET_DD) and c:IsLevelAbove(lvl)
    card::is_faceup(d, c) && !card::is_set_card(d, c, &[SET_DD as u16]) && card::is_level_above(d, c, lvl)
}
// s.sctg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local scale=e:GetHandler():GetLeftScale()
    let c = effect::get_handler(d, a.e).nil();
    let scale = card::get_left_scale(d, c);
    // local lvl=math.max(1,scale-2)
    let lvl = std::cmp::max(1, scale as i64 - 2);
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,0,nil,lvl)
    let g = duel::get_matching_group(d, Some(&|d, x| desfilter(d, x, lvl as u32)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if scale>1 then
    if scale > 1 {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
        let n = group::get_count(&g) as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    }
    true
}
// s.scop(e,tp,eg,ep,ev,re,r,rp)
fn scop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetLeftScale()<=1 then return end
    if card::get_left_scale(d, c) <= 1 {
        return;
    }
    // if c:UpdateScale(-2)~=0 then
    if card::update_scale(d, c, -2, None, None) != 0 {
        // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,0,nil,c:GetLeftScale())
        let lvl = card::get_left_scale(d, c);
        let g = duel::get_matching_group(d, Some(&|d, x| desfilter(d, x, lvl)), tp, LOCATION_MZONE, 0, NO_EX);
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
// s.rthfilter(c)
fn rthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DD) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DD as u16]) && card::is_able_to_hand(d, c, None)
}
// s.athfilter(c)
fn athfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DARK_CONTRACT) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_DARK_CONTRACT as u16]) && card::is_able_to_hand(d, c, None)
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and s.rthfilter(chkc) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && rthfilter(d, chkc) && chkc != c;
    }
    // local b1=Duel.IsExistingTarget(s.rthfilter,tp,LOCATION_ONFIELD,0,1,c)
    let b1 = duel::is_existing_target(d, Some(&rthfilter), a.tp, LOCATION_ONFIELD, 0, 1, c.into());
    // local b2=Duel.IsExistingMatchingCard(s.athfilter,tp,LOCATION_DECK,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&athfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.unwrap_or(0) as i64]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
        // local g=Duel.SelectTarget(tp,s.rthfilter,tp,LOCATION_ONFIELD,0,1,1,c)
        let g = duel::select_target(d, a.tp, Some(&rthfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into());
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    } else {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // e:SetProperty(EFFECT_FLAG_DELAY)
        effect::set_property(d, a.e, EFFECT_FLAG_DELAY, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    }
    true
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // Return 1 other "D/D" card you control to the hand
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,tc)
            duel::confirm_cards(d, 1 - tp, tc);
        }
    } else {
        // Search 1 "Dark Contract" card
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.athfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&athfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, &g);
        }
    }
}
