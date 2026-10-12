//! Zombyra
//! ゾンバイア
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 100459012;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card declares an attack: You lose 500 LP, and if you do, this card gains 500 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e1, Some(operation(e1_operation)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When a monster is destroyed by battle: You can equip this card from your GY to the monster that destroyed that monster, then you can destroy the 1 monster your opponent controls with the highest ATK (your choice, if tied)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_EQUIP+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_EQUIP + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return not eg:IsContains(e:GetHandler()) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        !group::is_contains(&a.eg.clone().nil(), h)
    })));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // The equipped monster's name becomes "Zombyra", also its effects are negated
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3a, EFFECT_TYPE_EQUIP);
    // e3a:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e3a, EFFECT_CHANGE_CODE);
    // e3a:SetValue(id)
    effect::set_value(d, e3a, ID as i64);
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local e3b=e3a:Clone()
    let e3b = effect::clone(d, e3a);
    // e3b:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e3b, EFFECT_DISABLE);
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
    // When the equipped monster declares an attack: Its controller loses 500 LP, and if they do, the equipped monster gains 500 ATK
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e4, CATEGORY_ATKCHANGE);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_condition(d, e4, Some(condition(e4_condition)));
    // e4:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e4, Some(operation(e4_operation)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}

/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and eg:GetFirst():GetBattleTarget():IsRelateToBattle() end
    if a.chk == Some(0) {
        let first = group::get_first(d, &a.eg.clone().nil()).nil();
        let bt = card::get_battle_target(d, first).nil();
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0 && card::is_relate_to_battle(d, bt);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}

/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=eg:GetFirst():GetBattleTarget()
    let bc = card::get_battle_target(d, group::get_first(d, &a.eg.clone().nil()).nil());
    // if c:IsRelateToEffect(e) and bc:IsRelateToBattle() and Duel.Equip(tp,c,bc,true) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_battle(d, bc.nil()) && duel::equip(d, tp, c, bc.nil(), true, false) {
        let bc = bc.nil();
        // Equip limit
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetValue(function(e,c) return c==bc end)
        effect::set_value_fn(d, e1, lua_fn(move |_d, a| a.card(1) == Some(bc)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // if Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x)), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
        {
            // local dg=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil):GetMaxGroup(Card.GetAttack)
            let all = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), tp, 0, LOCATION_MZONE, NO_EX);
            let mut dg = group::get_max_group(d, &all, &|d, x| card::get_attack(d, x) as i64).map_or_else(CardSet::new, |x| x.0);
            // if #dg>1 then
            if dg.len() > 1 {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
                // dg=dg:Select(tp,1,1,nil)
                dg = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
                // Duel.HintSelection(dg)
                duel::hint_selection(d, dg.clone(), true);
            }
            // if #dg>0 then
            if !dg.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Destroy(dg,REASON_EFFECT)
                duel::destroy(d, dg, REASON_EFFECT, None, None);
            }
        }
    }
}

/// `s.e1_operation`: `function(e,tp,eg,ep,ev,re,r,rp)`
fn e1_operation(d: &mut Duel, a: &EffArgs) {
    // local lp=Duel.GetLP(tp)
    let lp = duel::get_lp(d, a.tp);
    // Duel.SetLP(tp,lp-500)
    duel::set_lp(d, a.tp, lp - 500);
    // if Duel.GetLP(tp)<lp then
    if duel::get_lp(d, a.tp) < lp {
        // e:GetHandler():UpdateAttack(500)
        let h = effect::get_handler(d, a.e).nil();
        card::update_attack(d, h, 500, None, None, None);
    }
}

/// The condition of e4: `local c=e:GetHandler() return c:IsEquipSpell() and c:GetEquipTarget()==eg:GetFirst()`
fn e4_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_equip_spell(d, c) && card::get_equip_target(d, c) == group::get_first(d, &a.eg.clone().nil())
}

/// `s.e4_operation`: `function(e,tp,eg,ep,ev,re,r,rp)`
fn e4_operation(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetEquipTarget()
    // if not bc then return end
    let Some(bc) = card::get_equip_target(d, c) else { return };
    // local cntrl=bc:GetControler()
    let cntrl = card::get_controler(d, bc);
    // local lp=Duel.GetLP(cntrl)
    let lp = duel::get_lp(d, cntrl);
    // Duel.SetLP(cntrl,lp-500)
    duel::set_lp(d, cntrl, lp - 500);
    // if Duel.GetLP(cntrl)<lp then
    if duel::get_lp(d, cntrl) < lp {
        // bc:UpdateAttack(500,RESET_EVENT|RESETS_STANDARD,c)
        card::update_attack(d, bc, 500, Some(RESET_EVENT | RESETS_STANDARD), Some(c), None);
    }
}
