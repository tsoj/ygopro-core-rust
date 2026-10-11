//! Orgoth the Relentless
use super::prelude::*;

pub const ID: u32 = 15744417;
pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply appropriate effect, depending on dice results
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DICE+CATEGORY_ATKCHANGE+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DICE + CATEGORY_ATKCHANGE + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,3)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 3);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local res,atk={0,0,0,0,0,0,0,false,false,false,false},0
    // res[1..6]: the number of dice showing each face; res[8..10] (the "pairs" flags) and res[11] (the last die came up 3 times)
    let mut res = [0u32; 7];
    let mut r8 = false;
    let mut r9 = false;
    let mut r10 = false;
    let mut r11 = false;
    let mut atk: i64 = 0;
    // for _,i in ipairs({Duel.TossDice(tp,3)}) do
    for i in duel::toss_dice(d, a.tp, 3, 0) {
        let i = i as usize;
        // atk=atk+(i*100)
        atk += (i * 100) as i64;
        // res[i]=res[i]+1
        res[i] += 1;
        // if res[i]>=2 then res[(i+1)//2+7]=true end
        if res[i] >= 2 {
            match (i + 1) / 2 {
                1 => r8 = true,
                2 => r9 = true,
                _ => r10 = true,
            }
        }
        // res[11]=res[i]==3
        r11 = res[i] == 3;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(atk)
    effect::set_value(d, e1, atk);
    // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END,2)
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 2);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // if res[1+7] or res[11] then
    if r8 || r11 {
        // Cannot be destroyed by battle or card effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3008)
        effect::set_description(d, e1, 3008);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END,2)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 2);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
    // if res[2+7] or res[11] then
    if r9 || r11 {
        // Draw 2 cards
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, a.tp, 2, REASON_EFFECT);
    }
    // if res[3+7] or res[11] then
    if r10 || r11 {
        // Can attack directly
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetDescription(3205)
        effect::set_description(d, e3, 3205);
        // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_DIRECT_ATTACK)
        effect::set_code(d, e3, EFFECT_DIRECT_ATTACK);
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
    }
}
