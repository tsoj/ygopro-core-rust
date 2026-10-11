//! Mortilux Heruvur
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 15415552;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 8 monsters
    // Xyz.AddProcedure(c,nil,8,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // If a monster(s) is sent to your opponent's GY (except during the Damage Step): You can target 1 of them; attach it to this card. You can only use this effect of "Mortilux Heruvur" once per turn
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e1a, CATEGORY_LEAVE_GRAVE);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1a:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e1a, EVENT_CUSTOM + ID);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetTarget(s.attachtg)
    effect::set_target(d, e1a, Some(cost_target(attachtg)));
    // e1a:SetOperation(s.attachop)
    effect::set_operation(d, e1a, Some(operation(attachop)));
    // e1a:SetLabelObject(Group.CreateGroup())
    effect::set_label_object(d, e1a, Value::Group(CardSet::new()));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // Keep track of monsters sent to the opponent's GY
    // local e1b=Effect.CreateEffect(c)
    let e1b = effect::create_effect(d, c);
    // e1b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1b:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1b, EVENT_TO_GRAVE);
    // e1b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1b, LOCATION_MZONE);
    // e1b:SetLabelObject(e1a)
    effect::set_label_object(d, e1b, Value::Effect(e1a));
    // e1b:SetOperation(s.regop)
    effect::set_operation(d, e1b, Some(operation(regop)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // This card gains effects based on the number of materials attached to it
    // ● 2+: Cannot be destroyed by battle or card effects
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE);
    // e2a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2a:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2a, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetCondition(s.xyzmatcountcon(2))
    effect::set_condition(d, e2a, Some(lua_fn(|d, a: Args| overlay_count_at_least(d, a, 2))));
    // e2a:SetValue(1)
    effect::set_value(d, e2a, 1);
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2b, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // ● 3+: Your opponent cannot target cards in the GYs with card effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_GRAVE,LOCATION_GRAVE)
    effect::set_target_range(d, e3, LOCATION_GRAVE, LOCATION_GRAVE);
    // e3:SetCondition(s.xyzmatcountcon(3))
    effect::set_condition(d, e3, Some(lua_fn(|d, a: Args| overlay_count_at_least(d, a, 3))));
    // e3:SetValue(aux.tgoval)
    effect::set_value_fn(d, e3, aux::tgoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // ● 4+: You can detach 3 materials from this card; send 1 monster on the field to the GY
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e4, CATEGORY_TOGRAVE);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.xyzmatcountcon(4))
    effect::set_condition(d, e4, Some(lua_fn(|d, a: Args| overlay_count_at_least(d, a, 4))));
    // e4:SetCost(Cost.DetachFromSelf(3))
    effect::set_cost(d, e4, Some(cost::detach_from_self(cost::Count::N(3), None, None)));
    // e4:SetTarget(s.sendtogytg)
    effect::set_target(d, e4, Some(cost_target(sendtogytg)));
    // e4:SetOperation(s.sendtogyop)
    effect::set_operation(d, e4, Some(operation(sendtogyop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.xyzmatcountcon(required)`: `function(e) return e:GetHandler():GetOverlayCount()>=required end`
fn overlay_count_at_least(d: &mut Duel, a: Args, required: usize) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_overlay_count(d, h) >= required
}
/// `s.attachfilter(c,tp,e,xyzc)`
fn attachfilter(d: &mut Duel, c: CardId, tp: u8, e: EffectId, xyzc: CardId) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsMonster() and c:IsControler(1-tp)
    //     and c:IsCanBeXyzMaterial(xyzc,tp,REASON_EFFECT) and c:IsCanBeEffectTarget(e)
    card::is_location(d, c, LOCATION_GRAVE)
        && card::is_monster(d, c)
        && card::is_controler(d, c, 1 - tp)
        && card::is_can_be_xyz_material(d, c, Some(xyzc), Some(tp), Some(REASON_EFFECT))
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsDamageStep() then return end
    if duel::is_damage_step(d, None) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tg=eg:Filter(s.attachfilter,nil,tp,e,c)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, x| attachfilter(d, x, a.tp, a.e, c), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do tc:RegisterFlagEffect(id,RESET_CHAIN,0,1) end
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e1a = effect::get_label_object(d, a.e).as_effect().nil();
        let mut g = effect::get_label_object(d, e1a).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, x| card::get_flag_effect(d, x, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e1a, Value::Group(g));
        // Duel.RaiseSingleEvent(c,EVENT_CUSTOM+id,e,0,tp,tp,0)
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, a.tp, 0);
    }
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=e:GetLabelObject():Filter(s.attachfilter,nil,tp,e,c)
    let lg = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &lg, &|d, x| attachfilter(d, x, tp, a.e, c), NO_EX);
    // if chk==0 then return c:IsXyzMonster() and #g>0 end
    if a.chk == Some(0) {
        return card::is_xyz_monster(d, c) && !g.is_empty();
    }
    // local tc=nil
    // if #g==1 then
    let target_set = if g.len() == 1 {
        // tc=g:GetFirst()
        let tc = group::get_first(d, &g).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc);
        CardSet::from_card(tc)
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
        // tc=g:Select(tp,1,1,nil)
        let tc = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc.clone());
        tc
    };
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(target_set), 1, tp, 0);
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and tc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT) then
    if card::is_relate_to_effect(d, c, a.e) {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) && card::is_can_be_xyz_material(d, tc, Some(c), Some(a.tp), Some(REASON_EFFECT)) {
            // Duel.Overlay(c,tc)
            duel::overlay(d, c, tc, false);
        }
    }
}
/// `s.sendtogytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sendtogytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), 1, tp, 0);
    true
}
/// `s.sendtogyop(e,tp,eg,ep,ev,re,r,rp)`
fn sendtogyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGrave,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
