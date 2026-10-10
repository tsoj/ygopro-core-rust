//! Primathmech Laplacian
//! 塊斬機ラプラシア
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 88021907;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MATHMECH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,3)
    xyz::add_procedure(d, c, None, Some(4), 3, None, None, None, None, false, None);
    // If a "Mathmech" card(s) you control would be destroyed by card effect, you can detach 1 material from this card instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.reptg)
    effect::set_target(d, e1, Some(cost_target(reptg)));
    // e1:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e1, effect_card_fn(|d, e, c| {
        let h = effect::get_handler(d, e).nil();
        let tp = card::get_controler(d, h);
        repfilter(d, c, tp)
    }));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate a number of effects to activate equal to the number of materials detached
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCost(Cost.DetachFromSelf(1,s.effcostmax))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::F(std::sync::Arc::new(effcostmax))), None)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsXyzSummoned() end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h)
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MATHMECH) and c:IsControler(tp) and c:IsOnField() and c:IsFaceup()
    //   and c:IsReason(REASON_EFFECT) and not c:IsReason(REASON_REPLACE)
    card::is_set_card(d, c, &[SET_MATHMECH as u16])
        && card::is_controler(d, c, tp)
        && card::is_on_field(d, c)
        && card::is_faceup(d, c)
        && card::is_reason(d, c, REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp) and c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, x| repfilter(d, x, tp), 1, NO_EX) && card::check_remove_overlay_card(d, c, tp, 1, REASON_EFFECT);
    }
    // return Duel.SelectEffectYesNo(tp,c,96) and c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)>0
    duel::select_effect_yes_no(d, tp, c, Some(96)) && card::remove_overlay_card(d, c, tp, 1, 1, REASON_EFFECT) > 0
}
/// `s.effcostmax(e,tp)`: the most materials the cost can detach (one per kind of card to send)
fn effcostmax(d: &mut Duel, _e: EffectId, tp: u8) -> i64 {
    // local ct=0
    let mut ct = 0;
    // if Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,0,LOCATION_HAND,1,nil) then ct=ct+1 end
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_HAND, 1, NO_EX) {
        ct += 1;
    }
    // if Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,0,LOCATION_MZONE,1,nil) then ct=ct+1 end
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_MZONE, 1, NO_EX) {
        ct += 1;
    }
    // if Duel.IsExistingMatchingCard(aux.AND(Card.IsSpellTrap,Card.IsAbleToGrave),tp,0,LOCATION_ONFIELD,1,nil) then ct=ct+1 end
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_spell_trap(d, c) && card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX) {
        ct += 1;
    }
    // return ct
    ct
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local b1=Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,0,LOCATION_HAND,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_HAND, 1, NO_EX);
    // local b2=Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,0,LOCATION_MZONE,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b3=Duel.IsExistingMatchingCard(aux.AND(Card.IsSpellTrap,Card.IsAbleToGrave),tp,0,LOCATION_ONFIELD,1,nil)
    let b3 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_spell_trap(d, c) && card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    // local options={ {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)}, {b3,aux.Stringid(id,3)} }
    let mut options = [(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))];
    // local locations=0
    let mut locations: u32 = 0;
    // local cd=e:GetChainData()
    // local ct=#cd.cost_detached_materials
    let ct = chain::effect_data(d, a.e, None, chain::field::COST_DETACHED_MATERIALS).as_group().cloned().nil().len();
    // cd.selected_effects={}
    // (kept as a bit set: bit op is set when selected_effects[op] is true)
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(0));
    // for i=1,ct do
    for _ in 0..ct {
        // local op=Duel.SelectEffect(tp,table.unpack(options))
        let op = duel::select_effect(d, tp, &options).nil();
        // options[op][1]=false
        options[op - 1].0 = false;
        // cd.selected_effects[op]=true
        let sel = chain::effect_data(d, a.e, None, chain::field::CARD).int();
        chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(sel | (1 << op)));
        // locations=locations|((op==1 and LOCATION_HAND) or (op==2 and LOCATION_MZONE) or LOCATION_ONFIELD)
        locations |= match op {
            1 => LOCATION_HAND,
            2 => LOCATION_MZONE,
            _ => LOCATION_ONFIELD,
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,ct,1-tp,locations)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, ct as u8, 1 - tp, locations as i32);
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local break_chk=false
    let mut break_chk = false;
    // local cd=e:GetChainData()
    let sel = chain::effect_data(d, a.e, None, chain::field::CARD).int();
    // if cd.selected_effects[1] then
    if sel & (1 << 1) != 0 {
        // Send 1 random card from your opponent's hand to the GY
        // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,0,LOCATION_HAND,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_HAND, NO_EX);
        // if #g>0 then
        if !g.is_empty() {
            // break_chk=true
            break_chk = true;
            // local sg=g:RandomSelect(1-tp,1)
            let sg = group::random_select(d, &g, 1 - tp, 1);
            // Duel.SendtoGrave(sg,REASON_EFFECT)
            duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
        }
    }
    // if cd.selected_effects[2] then
    if sel & (1 << 2) != 0 {
        // Send 1 monster your opponent controls to the GY
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGrave,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
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
    // if cd.selected_effects[3] then
    if sel & (1 << 3) != 0 {
        // Send 1 Spell/Trap your opponent controls to the GY
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.AND(Card.IsSpellTrap,Card.IsAbleToGrave),tp,0,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_spell_trap(d, c) && card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // if break_chk then Duel.BreakEffect() end
            if break_chk {
                duel::break_effect(d);
            }
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    }
}
