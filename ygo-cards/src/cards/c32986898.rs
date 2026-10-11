//! Steel Star Regulator
use super::prelude::*;

pub const ID: u32 = 32986898;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon
    // Link.AddProcedure(c,s.matfilter,3,3)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 3, Some(3), None, None, None);
    // ATK increase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
    // e1:SetValue(s.matcheck)
    effect::set_value_fn(d, e1, lua_fn(matcheck));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy and damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,lc,st,tp)`
fn matfilter(d: &mut Duel, c: CardId, sc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return not c:IsType(TYPE_LINK,lc,st,tp)
    !card::is_type_ctx(d, c, TYPE_LINK, sc, st, Some(tp))
}
/// `s.mfilter(c)`
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsType(TYPE_XYZ)
    card::is_monster(d, c) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.matcheck(e,c)`
fn matcheck(d: &mut Duel, a: Args) {
    let c = a.c(1);
    // local g=c:GetMaterial():Filter(aux.NOT(Card.IsType),nil,TYPE_LINK)
    let mat = card::get_material(d, c);
    let g = group::filter(d, &mat, &|d, x| !card::is_type(d, x, TYPE_LINK), NO_EX);
    // local atk=g:GetSum(Card.GetOriginalLevel)+g:GetSum(Card.GetOriginalRank)
    let mut atk: i64 = 0;
    for tc in group::iter(d, &g) {
        atk += card::info_level(d, tc, None) as i64;
    }
    for tc in group::iter(d, &g) {
        atk += card::get_original_rank(d, tc) as i64;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(atk*100)
    effect::set_value(d, e1, atk * 100);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local tp=c:GetControler()
    let _tp = card::get_controler(d, c);
    // if g:IsExists(s.mfilter,1,nil) then
    if group::is_exists(d, &g, &|d, x| mfilter(d, x), 1, NO_EX) {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_TOFIELD|RESET_LEAVE|RESET_TEMP_REMOVE),EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,1))
        card::register_flag_effect(
            d,
            c,
            ID,
            RESET_EVENT | (RESETS_STANDARD & !(RESET_TOFIELD | RESET_LEAVE | RESET_TEMP_REMOVE)),
            EFFECT_FLAG_CLIENT_HINT,
            1,
            0,
            aux::stringid(ID, 1),
        );
    }
}
/// `s.desfilter(c,atk)`
fn desfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return not c:IsType(TYPE_LINK) and c:GetAttack()<=atk and c:IsFaceup()
    !card::is_type(d, c, TYPE_LINK) && card::get_attack(d, c) <= atk && card::is_faceup(d, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local atk=e:GetHandler():GetAttack()
    let h = effect::get_handler(d, a.e).nil();
    let atk = card::get_attack(d, h);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.desfilter(chkc,atk) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && desfilter(d, chkc, atk);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,0,LOCATION_MZONE,1,nil,atk) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| desfilter(d, c, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,0,LOCATION_MZONE,1,1,nil,atk)
    let g = duel::select_target(d, tp, Some(&|d, c| desfilter(d, c, atk)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0
    //     and e:GetHandler():GetFlagEffect(id)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
        let h = effect::get_handler(d, a.e).nil();
        if card::get_flag_effect(d, h, ID) > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local atk=tc:GetBaseAttack()/2
            let mut atk = card::get_base_attack(d, tc) as f64 / 2.0;
            // if atk<0 then atk=0 end
            if atk < 0.0 {
                atk = 0.0;
            }
            // Duel.Damage(1-tp,atk,REASON_EFFECT)
            duel::damage(d, 1 - tp, atk.round() as i64, REASON_EFFECT, false, None);
        }
    }
}
