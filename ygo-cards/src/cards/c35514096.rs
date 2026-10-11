//! Lord British Space Fighter
//! ロードブリティッシュ
use super::prelude::*;

pub const ID: u32 = 35514096;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[35514097], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card destroys an opponent's monster by battle: Activate 1 of these effects;
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetCondition(function(e) return e:GetHandler():IsRelateToBattle() end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e) return e:GetHandler():IsRelateToBattle() end`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_relate_to_battle(d, h)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsFacedown() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_facedown(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // --● This card can make a second attack in a row
    // local b1=e:GetHandler():CanChainAttack()
    let h = effect::get_handler(d, a.e).nil();
    let b1 = card::can_chain_attack(d, h, Some(2), false);
    // --● Target 1 Set card on the field; destroy that target
    // local b2=Duel.IsExistingTarget(Card.IsFacedown,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let b2 = duel::is_existing_target(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    // --● Special Summon 1 "Multi Token" (Machine/LIGHT/Level 4/ATK 1200/DEF 1200)
    // local b3=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_MULTI,0,TYPES_TOKEN,1200,1200,4,RACE_MACHINE,ATTRIBUTE_LIGHT)
    let b3 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && multi_token_summonable(d, tp);
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]);
    // if op then
    //     e:SetLabel(op)
    // end
    if let Some(op) = op {
        effect::set_label(d, a.e, &[op as i64]);
    }
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
    // elseif op==2 then
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectTarget(tp,Card.IsFacedown,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_target(d, tp, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    // elseif op==3 then
    } else if op == Some(3) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, tp, 0);
    }
    true
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_MULTI,0,TYPES_TOKEN,1200,1200,4,RACE_MACHINE,ATTRIBUTE_LIGHT)`
fn multi_token_summonable(d: &mut Duel, tp: u8) -> bool {
    duel::is_player_can_special_summon_monster(
        d,
        tp,
        ID + 1,
        duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(1200),
            defense: Some(1200),
            level: Some(4),
            race: Some(RACE_MACHINE),
            attribute: Some(ATTRIBUTE_LIGHT),
        },
        None,
        None,
        0,
    )
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // --● This card can make a second attack in a row
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:IsRelateToBattle() then
        if card::is_relate_to_battle(d, c) {
            // Duel.ChainAttack()
            duel::chain_attack(d, None);
        }
    // elseif op==2 then
    } else if op == 2 {
        // --● Target 1 Set card on the field; destroy that target
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) and tc:IsFacedown() then
        if card::is_relate_to_effect(d, tc, a.e) && card::is_facedown(d, tc) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    // elseif op==3 then
    } else if op == 3 {
        // --● Special Summon 1 "Multi Token" (Machine/LIGHT/Level 4/ATK 1200/DEF 1200)
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_MULTI,0,TYPES_TOKEN,1200,1200,4,RACE_MACHINE,ATTRIBUTE_LIGHT) then
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && multi_token_summonable(d, tp) {
            // local token=Duel.CreateToken(tp,TOKEN_MULTI)
            let token = duel::create_token(d, tp, ID + 1).nil();
            // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
